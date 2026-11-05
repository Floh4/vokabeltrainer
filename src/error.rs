use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum VocabError {
    DBError(rusqlite::Error),
    IoError(std::io::Error),
}

impl Display for VocabError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VocabError::DBError(e) => e.fmt(f),
            VocabError::IoError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for VocabError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VocabError::DBError(e) => Some(e),
            VocabError::IoError(e) => Some(e),
        }
    }
}
