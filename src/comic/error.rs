use std::num::ParseIntError;

use thiserror::Error;

use crate::comic::FormatError;

#[derive(Debug, Error)]
/// Errors that can occur when parsing a comic book file name.
pub enum ComicError {
    #[error("invalid input: no capture groups matched")]
    GetCaptures,
    #[error("failed to parse series name")]
    ParseSeries,
    #[error("failed to parse issue number")]
    ParseNumber(#[source] ParseIntError),
    #[error("failed to parse issue of number")]
    ParseOf(#[source] ParseIntError),
    #[error("failed to parse year")]
    ParseYear(#[source] ParseIntError),
    #[error("failed to get format")]
    GetFormat,
    #[error("failed to parse format")]
    ParseFormat(#[source] FormatError),
}
