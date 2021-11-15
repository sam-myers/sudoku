use std::error;
use std::fmt;

use crate::error::InvalidPuzzle;

#[derive(Debug, Clone)]
pub enum ImportError {
    Corruption,
    FileAccess,
    InvalidPuzzle,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImportError::Corruption => write!(f, "File is malformed"),
            ImportError::FileAccess => write!(f, "Can't read file"),
            ImportError::InvalidPuzzle => write!(f, "Not a valid sudoku puzzle"),
        }
    }
}

impl error::Error for ImportError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<InvalidPuzzle> for ImportError {
    fn from(_: InvalidPuzzle) -> Self {
        ImportError::InvalidPuzzle
    }
}

impl From<std::io::Error> for ImportError {
    fn from(_: std::io::Error) -> Self {
        ImportError::FileAccess
    }
}
