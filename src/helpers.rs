use crate::board::Board;
use crate::error::{Result, SudokuError};
use crate::importer::importer::Importer;
use crate::SDKImporter;
use std::fs::File;
use std::path::Path;

pub(crate) fn open_file(filename: &str) -> Result<File> {
    let path = Path::new(filename);
    File::open(&path).map_err(|_| SudokuError::FileRead(filename.to_string()))
}

#[allow(dead_code)]
pub(crate) fn get_test(test: &str) -> Result<Board> {
    let filename: String = format!("puzzles/tests/{}.sdk", test);
    let mut file = open_file(filename.as_str())?;
    SDKImporter.parse(&mut file)
}
