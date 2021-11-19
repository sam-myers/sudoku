use std::fs::File;
use std::path::Path;

use crate::board::Board;
use crate::error::Result;
use crate::helpers::open_file;
use crate::importer::Importer;
use crate::importer::SDKImporter;

#[allow(dead_code)]
pub fn get_test(test: &str) -> Result<Board> {
    let filename: String = format!("puzzles/tests/{}.sdk", test);
    let mut file = open_file(filename.as_str())?;
    SDKImporter.parse(&mut file)
}
