use std::fmt::Display;
use std::str::FromStr;

pub use error::FormatError;

mod error;

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    Cb7,
    Cbr,
    Cbt,
    Cbz,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cb7 => "cb7",
                Self::Cbr => "cbr",
                Self::Cbt => "cbt",
                Self::Cbz => "cbz",
            }
        )
    }
}

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
