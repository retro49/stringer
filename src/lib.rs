pub mod args;
pub mod config;
pub mod error;

use std::io::{self, Read};
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
    /// used for saving the current position of the buffer
    save: u64,
    /// size of the input buffer
    size: u64,
    /// checks if the buffer is at the end or not
    end: bool,
    /// result of the extracted string
    pub results: Vec<StringerResult>,
}

/// A result type where the extracted string and length of
/// the string is stored if needed
#[derive(Debug)]
pub struct StringerResult {
    /// the String that is extracted
    string: std::ffi::CString,
    /// size of the string if needed
    length: Option<u64>,
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
            save: 0,
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

    /// saves or stores the current position
    fn save(self: &mut Self) {
        self.save = self.pos;
    }

    /// restores the current position from save point
    fn restore(self: &mut Self) {
        self.pos = self.save;
    }

    /// This function is responsible for chekcing
    /// if the current byte is acceptable or not according
    /// to the rule provided by the configuration.
    fn should_read(self: &Self) -> bool {
        if  self.byte.is_ascii_alphanumeric() {
                return true;
        }

        if !self.config.special &&  
            self.byte.is_ascii_punctuation() {
                return false;
        }

        if !self.config.whitespace_include && 
            self.byte.is_ascii_whitespace()  {
                return false;
        }

        return true;
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

        if (self.config.window_max_size < self.config.window_min_size) ||
            self.config.window_max_size == 0 {
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
            for _ in 0..self.config.window_max_size {
                if !self.should_read() || self.end {
                    break;
                }
                buff.push(self.byte);
                self.advance();
            }
        }

        if buff.len() < self.config.window_min_size as usize {
            if self.pos >= self.size - 1{
                self.end = true;
            }
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
                            println!("{:?}", r);
                            self.results.push(r);
                        },
                        None => {continue;},
                    };
                },
                None => { continue; }
            };

        }
    }
}
