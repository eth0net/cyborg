use crate::comic::error::ComicError;
use crate::format::Format;

pub(super) fn series(captures: &regex::Captures) -> Result<String, ComicError> {
    captures
        .name("series")
        .ok_or(ComicError::ParseSeries)
        .map(|s| s.as_str().to_string())
}

pub(super) fn number(captures: &regex::Captures) -> Result<Option<usize>, ComicError> {
    captures
        .name("number")
        .map(|n| n.as_str().parse::<usize>().map_err(ComicError::ParseNumber))
        .transpose()
}

pub(super) fn suffix(captures: &regex::Captures) -> Option<String> {
    captures.name("suffix").map(|s| s.as_str().to_string())
}

pub(super) fn of(captures: &regex::Captures) -> Result<Option<usize>, ComicError> {
    captures
        .name("of")
        .map(|o| o.as_str().parse::<usize>().map_err(ComicError::ParseOf))
        .transpose()
}

pub(super) fn title(captures: &regex::Captures) -> Option<String> {
    captures.name("title").map(|t| t.as_str().to_string())
}

pub(super) fn year(captures: &regex::Captures) -> Result<Option<usize>, ComicError> {
    captures
        .name("year")
        .map(|y| y.as_str().parse::<usize>().map_err(ComicError::ParseYear))
        .transpose()
}

pub(super) fn tags(captures: &regex::Captures) -> Vec<String> {
    captures
        .name("tags")
        .map(|t| {
            t.as_str()
                .split(['(', ')'])
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn format(captures: &regex::Captures) -> Result<Format, ComicError> {
    captures
        .name("format")
        .ok_or(ComicError::GetFormat)?
        .as_str()
        .parse::<Format>()
        .map_err(ComicError::ParseFormat)
}
