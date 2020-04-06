use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ImportError;

const DESCRIPTION: &'static str = "unable to import puzzle";

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, DESCRIPTION)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ImportError {
    fn description(&self) -> &str { DESCRIPTION }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
