use std::fs::File;
use std::path::Path;
use crate::error::{Result, SudokuError};

pub fn open_file(filename: &str) -> Result<File> {
    let path = Path::new(filename);
    File::open(&path).map_err(|_| SudokuError::FileRead(filename.to_string()))
}
