use std::ffi::OsStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    Cb7,
    Cbr,
    Cbt,
    Cbz,
}

impl TryFrom<&OsStr> for Format {
    type Error = String;

    fn try_from(s: &OsStr) -> Result<Self, Self::Error> {
        s.to_str().map(Self::try_from).ok_or("invalid format")?
    }
}

impl TryFrom<&str> for Format {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "cb7" => Ok(Self::Cb7),
            "cbr" => Ok(Self::Cbr),
            "cbt" => Ok(Self::Cbt),
            "cbz" => Ok(Self::Cbz),
            _ => Err("invalid format".to_string()),
        }
    }
}
