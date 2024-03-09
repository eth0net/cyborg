use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct FormatError;

impl Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid format")
    }
}

impl Error for FormatError {}
