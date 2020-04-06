use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, InvalidPuzzle>;

#[derive(Debug, Clone)]
pub struct InvalidPuzzle;

impl fmt::Display for InvalidPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "this is not a valid sudoku puzzle")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for InvalidPuzzle {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
