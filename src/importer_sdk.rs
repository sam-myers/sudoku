use crate::board::Board;
use crate::error_import::*;
use crate::importer::Importer;
use crate::num::Num;

use std::error;
use std::io::BufRead;
use std::vec::Vec;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct SDKImporter;

impl Importer for SDKImporter {
    fn parse<R: BufRead>(reader: &mut R) -> Result<Board> {
        let mut bytes: Vec<u8> = Vec::with_capacity(91);
        reader.read_to_end(&mut bytes);

        let mut board = Board::new();

        let mut index = 0;
        for x in 0..9 {
            for y in 0..9 {
                match bytes[index] as char {
                    '1'..'9' => { board = board.given(x, y, Num::from_int(bytes[index]-48).expect("SDKImporter bug"))?; index += 1; },
                    '.' => index += 1,
                    '\n' => (),
                    '#' => return Err(Box::New(ImportError)), // Comments not supported
                    _ => return Err(ImportError), // Unknown field
                }
            }
        }

        if index == 81 {
            Ok(board)
        } else {
            Err(ImportError)
        }
    }
}
