use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    Cb7,
    Cbr,
    Cbt,
    Cbz,
}

#[derive(Debug)]
pub struct FormatError;

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid format")
    }
}

impl Error for FormatError {}

impl FromStr for Format {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cb7" => Ok(Self::Cb7),
            "cbr" => Ok(Self::Cbr),
            "cbt" => Ok(Self::Cbt),
            "cbz" => Ok(Self::Cbz),
            _ => Err(FormatError),
        }
    }
}
