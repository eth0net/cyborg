use once_cell::sync::Lazy;
use regex::Regex;

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
        \.(?<format>[Cc][Bb][7RrTtZz])
        $";

pub(super) static COMIC: Lazy<Regex> = Lazy::new(|| Regex::new(COMIC_REGEX).unwrap());
