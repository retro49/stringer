/// Error that is retruned by stringer
#[derive(Debug)]
pub struct StringerError {
    msg: String,
}

impl std::fmt::Display for StringerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StringerError {{ msg: {} }}", self.msg))
    }
}

/// Default stringer error
impl Default for StringerError {
    fn default() -> Self {
        StringerError {
            msg: "StringerError {}".to_string(),
        }
    }
}

impl StringerError {
    /// creates a new stringer error type with a message
    pub fn new(msg: String) -> StringerError {
        Self { msg }
    }
}

impl std::error::Error for StringerError {}
