use thiserror::Error;

pub type Result<T> = std::result::Result<T, SudokuError>;

#[derive(Error, Debug)]
pub enum SudokuError {
    #[error("Cannot read file {0}")]
    FileRead(String),

    #[error("Malformed file")]
    MalformedFile,

    #[error("Invalid puzzle")]
    InvalidPuzzle,

    #[error("Internal bug")]
    InternalConsistencyError,

    #[error("Could not solve puzzle")]
    UnsolvablePuzzle,
}
