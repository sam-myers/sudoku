use std::path::Path;
use std::fs::File;

use crate::board::Board;
use crate::error_import::ImportError;
use crate::importer::Importer;
use crate::importer_sdk::SDKImporter;

#[allow(dead_code)]
pub fn get_test(test: &str) -> Result<Board, ImportError> {
    let filename: String = format!("puzzles/tests/{}.sdk", test);
    let path = Path::new(filename.as_str());
    let mut file = File::open(&path)?;
    SDKImporter.parse(&mut file)
}
