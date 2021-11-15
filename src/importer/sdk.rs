use crate::board::Board;
use crate::error::ImportError;
use crate::importer::Importer;
use crate::num::Num;

use std::io::Read;
use std::vec::Vec;

pub struct SDKImporter;

impl Importer for SDKImporter {
    fn parse<R: Read>(&self, reader: &mut R) -> Result<Board, ImportError> {
        let mut bytes: Vec<u8> = Vec::with_capacity(91);
        reader.read_to_end(&mut bytes)?;

        let tiles: Vec<char> = bytes
            .iter()
            .map(|b| *b as char)
            .filter(|b| match b {
                '1'..='9' => true,
                '.' => true,
                _ => false,
            })
            .collect();

        let mut board = Board::new();

        let mut index = 0;
        for x in 0..9 {
            for y in 0..9 {
                let tile = tiles.get(index);
                if tile.is_none() {
                    return Err(ImportError::Corruption);
                }
                match *tile.unwrap() as char {
                    '1' => board = board.given(x, y, Num::One)?,
                    '2' => board = board.given(x, y, Num::Two)?,
                    '3' => board = board.given(x, y, Num::Three)?,
                    '4' => board = board.given(x, y, Num::Four)?,
                    '5' => board = board.given(x, y, Num::Five)?,
                    '6' => board = board.given(x, y, Num::Six)?,
                    '7' => board = board.given(x, y, Num::Seven)?,
                    '8' => board = board.given(x, y, Num::Eight)?,
                    '9' => board = board.given(x, y, Num::Nine)?,
                    '.' => (),
                    _ => return Err(ImportError::Corruption), // Should have been already filtered
                }
                index += 1;
            }
        }

        if index == 81 {
            Ok(board)
        } else {
            Err(ImportError::Corruption)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::get_test;

    #[test]
    fn test_corrupt_1() {
        assert!(get_test("corrupt_1").is_err());
    }
}
