use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::format::Format;

static COMIC_REGEX: &str = r"(?x)
    ^
    # Series name, or full name for non-serial comics.
    (?<series>[\w\s\#()&'+-.]+?)

    (?:\s+
        # Issue or volume number.
        \#?(?<number>\d+)

        # Issue of volume suffix.
        (?<suffix>\w+)?

        # Total issues in limited series.
        (?:\s+\(?[Oo][Ff]\s+\#?(?<of>\d+)\)?)?

        # Issue title.
        (?:\s+(?<title>[\w\s\#&'+-.]+?))?
    )?

    # Cover year.
    (?:\s+\((?<year>\d{4})\))?

    # Tags for the comic.
    (?:\s+(?<tags>(?:\s*\([\w\s\#&'+-.]+\))+))?
    $";

#[derive(Debug, PartialEq, Eq)]
pub struct Comic {
    // Name of the series.
    //
    // This should always be present as all comics,
    // even one-shots and volumes, are part of a series.
    pub series: String,

    // Issue or volume number.
    //
    // This should be present for regular issues and volumes,
    // but often not for one-shots or collected editions.
    //
    // Optional since not all comics have a number.
    pub number: Option<usize>,

    // Issue or volume suffix.
    //
    // This is often not present for regular issues,
    // but is often used for annuals and special issues.
    pub suffix: Option<String>,

    // Total number of issues in the series.
    //
    // This is often not present for regular issues,
    // but is usually present for limited series.
    pub of: Option<usize>,

    // Issue title.
    //
    // This is often not present for regular issues,
    // but is often present for one-shots and collected editions.
    pub title: Option<String>,

    // Year on the cover.
    //
    // This is nearly always present, but is optional
    // since year may not be known for all comics.
    pub year: Option<usize>,

    // Tags for the comic.
    //
    // This may or may not be present for all comics,
    // but is optional since not all comics have tags.
    pub tags: Vec<String>,

    // Format of the comic.
    pub format: Format,
}

impl TryFrom<&str> for Comic {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(COMIC_REGEX).unwrap());

        let path = PathBuf::from(s);

        let file_name = path
            .file_stem()
            .ok_or("getting file name")?
            .to_str()
            .ok_or("converting file name to string")?;
        let caps = RE.captures(file_name).ok_or("getting captures")?;

        let series = caps
            .name("series")
            .ok_or("getting series")?
            .as_str()
            .to_string();
        let number = caps.name("number").map(|n| n.as_str().parse().unwrap());
        let suffix = caps.name("suffix").map(|s| s.as_str().to_string());
        let of = caps.name("of").map(|n| n.as_str().parse().unwrap());
        let title = caps.name("title").map(|t| t.as_str().to_string());
        let year = caps.name("year").map(|y| y.as_str().parse().unwrap());
        let tags = caps
            .name("tags")
            .map(|t| {
                t.as_str()
                    .split(['(', ')'])
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let extension = path.extension().ok_or("getting extension")?;
        let format = Format::try_from(extension)?;

        Ok(Self {
            series,
            number,
            suffix,
            of,
            title,
            year,
            tags,
            format,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::format::Format;

    use super::*;

    #[test]
    fn comic_from_string() -> Result<(), String> {
        let cases = [
            (
                "Simple 001.cbr",
                Comic {
                    series: "Simple".into(),
                    number: Some(1),
                    suffix: None,
                    of: None,
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbr,
                },
            ),
            (
                "Year 090 (2024).cbt",
                Comic {
                    series: "Year".into(),
                    number: Some(90),
                    suffix: None,
                    of: None,
                    title: None,
                    year: Some(2024),
                    tags: vec![],
                    format: Format::Cbt,
                },
            ),
            (
                "Dashed - Series-Name 10.cbz",
                Comic {
                    series: "Dashed - Series-Name".into(),
                    number: Some(10),
                    suffix: None,
                    of: None,
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbz,
                },
            ),
            (
                "Tagged (Simple) (Over-Complicated Tag).cbr",
                Comic {
                    series: "Tagged".into(),
                    number: None,
                    suffix: None,
                    of: None,
                    title: None,
                    year: None,
                    tags: vec!["Simple".into(), "Over-Complicated Tag".into()],
                    format: Format::Cbr,
                },
            ),
            (
                "Year Tagged (2024) (Simple) (Over-Complicated Tag).cbt",
                Comic {
                    series: "Year Tagged".into(),
                    number: None,
                    suffix: None,
                    of: None,
                    title: None,
                    year: Some(2024),
                    tags: vec!["Simple".into(), "Over-Complicated Tag".into()],
                    format: Format::Cbt,
                },
            ),
            (
                "Special-characters - + & (x) (10) (+) '99 020.cbz",
                Comic {
                    series: "Special-characters - + & (x) (10) (+) '99".into(),
                    number: Some(20),
                    suffix: None,
                    of: None,
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbz,
                },
            ),
            (
                "Limited #01 of #02 (2020) (Tag).cbr",
                Comic {
                    series: "Limited".into(),
                    number: Some(1),
                    suffix: None,
                    of: Some(2),
                    title: None,
                    year: Some(2020),
                    tags: vec!["Tag".into()],
                    format: Format::Cbr,
                },
            ),
            (
                "Limited #02 (OF #03).cbt",
                Comic {
                    series: "Limited".into(),
                    number: Some(2),
                    suffix: None,
                    of: Some(3),
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbt,
                },
            ),
            (
                "Limited 003 OF #4.cbz",
                Comic {
                    series: "Limited".into(),
                    number: Some(3),
                    suffix: None,
                    of: Some(4),
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbz,
                },
            ),
            (
                "Limited 004 (of 5).cbr",
                Comic {
                    series: "Limited".into(),
                    number: Some(4),
                    suffix: None,
                    of: Some(5),
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbr,
                },
            ),
            (
                "Series With 001 Issue Name (2023).cbt",
                Comic {
                    series: "Series With".into(),
                    number: Some(1),
                    suffix: None,
                    of: None,
                    title: Some("Issue Name".into()),
                    year: Some(2023),
                    tags: vec![],
                    format: Format::Cbt,
                },
            ),
            (
                "With.Dots 001.cbr",
                Comic {
                    series: "With.Dots".into(),
                    number: Some(1),
                    suffix: None,
                    of: None,
                    title: None,
                    year: None,
                    tags: vec![],
                    format: Format::Cbr,
                },
            ),
            (
                "With Suffix 001X (2023).cbt",
                Comic {
                    series: "With Suffix".into(),
                    number: Some(1),
                    suffix: Some("X".into()),
                    of: None,
                    title: None,
                    year: Some(2023),
                    tags: vec![],
                    format: Format::Cbt,
                },
            ),
        ];

        for (input, expected) in cases {
            let comic = Comic::try_from(input).map_err(|e| format!("{}: {}", e, input))?;
            assert_eq!(comic, expected);
        }

        Ok(())
    }
}
