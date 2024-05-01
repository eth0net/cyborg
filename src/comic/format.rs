use std::fmt::Display;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid format")]
/// Error that occurs when parsing a comic book format.
pub struct FormatError;

#[derive(Debug, PartialEq, Eq)]
/// The format of a comic book file.
pub enum Format {
    /// 7z archive.
    Cb7,
    /// Rar archive.
    Cbr,
    /// Tar archive.
    Cbt,
    /// Zip archive.
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
