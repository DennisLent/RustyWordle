use std::fmt;

#[derive(Debug)]
pub enum DictionaryError {
    PathError,
    ParseError,
    DownloadError,
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DictionaryError::PathError => write!(f, "Dictionary Error: Path Error"),
            DictionaryError::ParseError => write!(f, "Dictionary Error: Parse Error"),
            DictionaryError::DownloadError => write!(f, "Dictionary Error: Download Error"),
        }
    }
}
