use std::error;
use std::fmt;

use crate::error::InvalidPuzzle;

#[derive(Debug, Clone)]
pub struct ImportError;

const DESCRIPTION: &str = "unable to import puzzle";

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", DESCRIPTION)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ImportError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str { DESCRIPTION }
}

impl From<InvalidPuzzle> for ImportError {
    fn from(_: InvalidPuzzle) -> Self {
        ImportError
    }
}

impl From<std::io::Error> for ImportError {
    fn from(_: std::io::Error) -> Self {
        ImportError
    }
}
