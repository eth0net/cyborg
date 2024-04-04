use std::num::ParseIntError;

use thiserror::Error;

use super::FormatError;

#[derive(Debug, Error)]
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
