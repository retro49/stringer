/// Output formats that are supported
pub enum OutputFormat {
    JSON,
    XML,
    Literal,
}

/// Default Window minimum size config
const WINDOW_MIN_SIZE: u64 = 4;
/// Default window maximum size config
const WINDOW_MAX_SIZE: u64 = 0;
/// Default special character config
const SPECIAL_INCLUDE: bool = true;
/// Default whitespace including config
const WHITESPACE_INCLUDE: bool = true;
/// Default output format config
const OUTPUTFORMAT: OutputFormat = OutputFormat::Literal;
/// Default string length  output format config
const LENGTH: bool = false;
/// Default regex option
const REGEX: Option<regex::Regex> = None;
/// By default does not treats LF CR as a 
/// valid character in the extraction process
const LINE_INCLUDE: bool = false;
/// Default split option, no splitting
const SPLIT: u64 = 0;

/// StringerConfig
/// This structure specifies the configuration
/// that are used during the string extraction
/// in order to customize the process.
pub struct StringerConfig {
    /// @window_min_size: the minimum size of a string length to start with
    pub window_min_size: u64,
    /// @window_max_size: the maximum size of a string length
    pub window_max_size: u64,
    /// @special: support also for special characters during extraction
    /// so that the extractor is also capable of including special characters
    /// in the string
    pub special: bool,
    /// @whitespace_include: sets the extractor to treat whitespace's as
    /// a character in the string
    pub whitespace_include: bool,
    /// @output_format: formatting method for printing the result
    pub output_format: OutputFormat,
    /// @length: include the size of the string length in the output
    pub length: bool,
    /// splits every string extracted into split capacity
    pub split: u64,
    /// @regex: regex pattern for matching, optional.
    pub regex: Option<regex::Regex>,
    /// @line_include: treats 0x0a LINE FEED, 0x0d CR as a string
    pub line_include: bool,
}

impl Default for StringerConfig {
    /// Default configuration for stringer
    fn default() -> Self {
        StringerConfig {
            window_min_size: WINDOW_MIN_SIZE,
            window_max_size: WINDOW_MAX_SIZE,
            special: SPECIAL_INCLUDE,
            whitespace_include: WHITESPACE_INCLUDE,
            output_format: OUTPUTFORMAT,
            length: LENGTH,
            regex: REGEX,
            line_include: LINE_INCLUDE,
            split: SPLIT,
        }
    }
}

impl StringerConfig {
    /// creates a new configuration that is the same as default
    pub fn new() -> Self {
        StringerConfig::default()
    }

    /// includes special characters in the extracted string
    pub fn special_include(self: &mut Self, opt: bool) {
        self.special = opt;
    }

    /// changes the minimum window size to provided size
    pub fn set_window_min_size(self: &mut Self, ws: u64) {
        self.window_min_size = ws;
    }

    /// changes the maximum window size to provided size
    pub fn set_window_max_size(self: &mut Self, ws: u64) {
        self.window_max_size = ws;
    }

    /// makes whitespace to be included in the extraction
    pub fn whitespace_include(self: &mut Self, opt: bool) {
        self.whitespace_include = opt;
    }

    /// sets the output format to speci
    pub fn set_output_format(self: &mut Self, opt: OutputFormat) {
        self.output_format = opt;
    }

    /// enable length to be included in the output
    pub fn length_include(self: &mut Self, opt: bool) {
        self.length = opt;
    }

    /// makes LINE FEED AND CR to be treated as string
    pub fn line_include(self: &mut Self, opt: bool) {
        self.line_include = opt;
    }

    /// splits the provided string into segments
    pub fn split(self: &mut Self, opt: u64) {
        self.split = opt;
    }

    /// set regex expression
    pub fn regex(self: &mut Self, opt: String) -> Result<(), crate::error::StringerError> {
        let reg = regex::Regex::new(&opt);
        match reg {
            Ok(r) => {
                self.regex = Some(r);
            }
            Err(_) => {
                return Err(crate::error::StringerError::new(
                    "unable to compile provided regex".to_string(),
                ));
            }
        };
        Ok(())
    }
}

impl From<crate::args::Args> for StringerConfig {
    /// Converts arguments provided to stringer into config
    /// Arguments that are not provided are set to their default values
    /// NOTE: If the provided regex is not valid then the regex will be empty.
    fn from(value: crate::args::Args) -> Self {
        let mut conf = StringerConfig::default();

        conf.set_window_min_size(match value.window_min_size {
            Some(s) => s,
            _ => WINDOW_MIN_SIZE,
        });

        conf.set_window_max_size(match value.window_max_size {
            Some(s) => s,
            _ => WINDOW_MAX_SIZE,
        });

        conf.special_include(match value.special {
            Some(s) => s,
            _ => SPECIAL_INCLUDE,
        });

        conf.whitespace_include(match value.whitespace {
            Some(w) => w,
            _ => WHITESPACE_INCLUDE,
        });

        conf.length_include(match value.length {
            Some(l) => l,
            _ => LENGTH,
        });

        conf.line_include(match value.line_include {
            Some(l) => { l },
            _ => LINE_INCLUDE
        });

        conf.split(match value.split{
            Some(s) => {s}
            _ => { SPLIT }
        });

        conf.output_format = match value.output_format {
            Some(f) => match f.to_lowercase().as_str() {
                "literal" => OutputFormat::Literal,
                "json" => OutputFormat::JSON,
                "xml" => OutputFormat::XML,
                _ => OutputFormat::Literal,
            },
            _ => OUTPUTFORMAT,
        };

        conf.regex = match value.regex {
            Some(r) => match regex::Regex::new(&r) {
                Ok(r) => Some(r),
                Err(_) => None,
            },
            _ => None,
        };
        return conf;
    }
}
