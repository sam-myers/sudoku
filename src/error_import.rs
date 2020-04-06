use std::error;
use std::fmt;

pub type ImportErrorResult<T> = std::result::Result<T, ImportError>;

#[derive(Debug, Clone)]
pub struct ImportError;

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to import puzzle")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ImportError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
