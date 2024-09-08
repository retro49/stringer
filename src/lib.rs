pub mod args;
pub mod config;
pub mod error;

use std::io::Read;
use serde::ser::{Serialize, SerializeStruct};

/// Stringer
/// The main structure that is used to extract
/// strings from a given stream according to the
/// provided config.
pub struct Stringer {
    /// A custom configuration for the extractor
    config: config::StringerConfig,
    /// input buffer where the stream is read and extracted
    buffer: Vec<u8>,
    /// the current byte in the stream
    byte: u8,
    /// the position of the byte or the cursor
    pos: u64,
    /// size of the input buffer
    size: u64,
    /// checks if the buffer is at the end or not
    end: bool,
    /// result of the extracted string
    pub results: Vec<StringerResult>,
}

/// A result type where the extracted string and length of
/// the string is stored if needed
// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Clone)]
pub struct StringerResult {
    /// the String that is extracted
    string: std::ffi::CString,
    /// size of the string if needed
    length: Option<u64>,
}

impl Serialize for StringerResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let mut ss = serializer.serialize_struct("StringerResult", 2)?;
                match self.length {
                    Some(n) => {
                        ss.serialize_field("length", &n)?;
                    }
                    _ => {}
                };
                ss.serialize_field("string", self.string.to_str().unwrap())?;
                ss.end()
    }
}

impl std::fmt::Display for StringerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.length {
            Some(n) => {
                let _ = f.write_fmt(format_args!("{}, ", n ));
            }
            None => {}
        };
        f.write_fmt(format_args!("{}", self.string.to_str().unwrap()))
    }
}

impl<'a> Stringer {
    /// creates a new stringer instance from objects that 
    /// implement the std::io::Read trait.
    /// Configuration is the default.
    pub fn new<R: ?Sized>(input: &mut R) -> 
        Result<Self, Box<dyn std::error::Error>> 
        where R: Read
    {
        let mut buff: Vec<u8> = Vec::<u8>::new();
        match input.read_to_end(&mut buff) {
            Err(e) => {
                return Err(Box::new(e));
            }
            _ => {}
        };

        Ok(Stringer {
            config: config::StringerConfig::default(),
            size: buff.len() as u64,
            byte: 'blk: {
                if buff.len() <= 0 {
                    break 'blk 0;
                }
                break 'blk buff[0];
            },
            end: 'blk: {
                if buff.len() == 0 {
                    break 'blk true;
                }
                break 'blk false;
            },
            buffer: buff,
            pos: 0,
            results: Vec::new(),
        })
    }

    pub fn set_config(self: &mut Self, config: crate::config::StringerConfig) {
        self.config = config;
    }

    /// Advances the cursor by one step
    /// If end is there then the end flag is set on
    fn advance(self: &mut Self) {
        if self.pos >= self.size - 1 {
            self.end = true;
            return;
        }

        self.pos += 1;
        self.byte = self.buffer[self.pos as usize];
    }

    /// This function is responsible for chekcing
    /// if the current byte is acceptable or not according
    /// to the rule provided by the configuration.
    fn should_read(self: &Self) -> bool {
        if  self.byte.is_ascii_alphanumeric() {
                return true;
        }

        if self.config.special &&  
            self.byte.is_ascii_punctuation() {
                return true;
        }

        if self.config.whitespace_include && 
            (self.byte == 0x20 || 
             self.byte == 0x09 || 
             self.byte == 0x0b
             ) 
        {
                return true;
        }

        if self.config.line_include && (self.byte == 0x0a || self.byte == 0x0d) {
            return true;
        }

        return false;
    }

    /// reads in and returns a buffer if the config condition
    /// is met and not the end is reached
    fn read(self: &mut Self) -> Option<Vec<u8>> {
        while !self.should_read() {
            self.advance();
            if self.end {
                return None;
            }
        }

        let mut buff: Vec<u8> = Vec::<u8>::new();

        if self.config.split == 0 {
            loop {
                if !self.should_read() {
                    break;
                }

                if self.end {
                    break;
                }

                buff.push(self.byte);
                self.advance();
            }
        } else {
            for _ in 0..self.config.split {
                if !self.should_read() || self.end {
                    break;
                }

                buff.push(self.byte);
                self.advance();
            }
        }

        if buff.len() < self.config.window_min_size as usize {
            return None;
        }

        // maximum window size limitation
        if self.config.window_max_size != 0 && 
           buff.len() as u64 > self.config.window_max_size {
            return None;
        }

        buff.push(0);
        Some(buff)
    }

    /// returns a stringer result type after reading
    /// based on the rule the string size is also returned if needed
    fn to_stringer(self: &mut Self, buff: Vec<u8>) -> Option<StringerResult> {
        let len = buff.len() as u64;
        let string = 
            std::ffi::CString::from_vec_with_nul(buff);

        match string {
            Err(_) => { return None; }
            Ok(s)  => {
                return Some(StringerResult {
                    length: match self.config.length {
                        true => { Some(len - 1) },
                        false => { None }
                    },
                    string: s
                });
            }
        };
    }

    /// reads all the available strings in the stream
    /// and stores them in results
    pub fn read_strings(self: &mut Self) {
        while !self.end {
            let buff = self.read();
            match buff {
                Some(buff) => {
                    let res = self.to_stringer(buff);
                    match res {
                        Some(r) => {
                            self.results.push(r);
                        },
                        None => {continue;},
                    };
                },
                None => { continue; }
            };

        }

        match self.config.regex{
            Some(ref r) => {
                // this clone is gonna cost a lot.
                let res: Vec<StringerResult> = 
                    self.results
                    .iter()
                    .filter({ |x|  
                        r.is_match(x.string.to_str().unwrap())
                    }).cloned()
                .collect();
                self.results = res;
            }
            _ => {}
        };
    }
}
