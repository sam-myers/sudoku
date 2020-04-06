use crate::board::Board;
use crate::error_import::*;
use crate::importer::Importer;

use std::io::BufRead;
use std::vec::Vec;

pub struct SDKImporter;

impl Importer for SDKImporter {
    fn parse<R: BufRead>(reader: &mut R) -> ImportErrorResult<Board> {
        let mut bytes: Vec<u8> = Vec::new();
        reader.read_to_end(&mut bytes);

        Err(ImportError)
    }
}
