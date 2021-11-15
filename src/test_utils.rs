use std::fs::File;
use std::path::Path;

use crate::board::Board;
use crate::error::ImportError;
use crate::importer::Importer;
use crate::importer::SDKImporter;

#[allow(dead_code)]
pub fn get_test(test: &str) -> Result<Board, ImportError> {
    let filename: String = format!("puzzles/tests/{}.sdk", test);
    let path = Path::new(filename.as_str());
    let mut file = File::open(&path)?;
    SDKImporter.parse(&mut file)
}
