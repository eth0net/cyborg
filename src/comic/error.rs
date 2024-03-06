use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;

use super::format::FormatError;

#[derive(Debug)]
pub enum ComicError {
    GetCaptures,
    ParseSeries,
    ParseNumber(ParseIntError),
    ParseOf(ParseIntError),
    ParseYear(ParseIntError),
    GetFormat,
    ParseFormat(FormatError),
}

impl Display for ComicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComicError::GetCaptures => write!(f, "invalid input: no capture groups matched"),
            ComicError::ParseSeries => write!(f, "failed to parse series name"),
            ComicError::ParseNumber(_) => write!(f, "failed to parse issue number"),
            ComicError::ParseOf(_) => write!(f, "failed to parse issue of number"),
            ComicError::ParseYear(_) => write!(f, "failed to parse year"),
            ComicError::GetFormat => write!(f, "failed to get format"),
            ComicError::ParseFormat(_) => write!(f, "failed to parse format"),
        }
    }
}

impl Error for ComicError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ComicError::GetCaptures => None,
            ComicError::ParseSeries => None,
            ComicError::ParseNumber(err) => Some(err),
            ComicError::ParseOf(err) => Some(err),
            ComicError::ParseYear(err) => Some(err),
            ComicError::GetFormat => None,
            ComicError::ParseFormat(err) => Some(err),
        }
    }
}
