use crate::board::Board;
use crate::digit::Digit;
use crate::error::{Result, SudokuError};
use crate::importer::importer::Importer;
use crate::tile::Tile;
use crate::tile_group::TileGroup;
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

#[allow(dead_code)]
pub(crate) fn assert_tile_equal(tile: &Tile, expected: &str) {
    match expected {
        "1" => assert_eq!(tile, &Tile::Known(Digit::new(1))),
        "2" => assert_eq!(tile, &Tile::Known(Digit::new(2))),
        "3" => assert_eq!(tile, &Tile::Known(Digit::new(3))),
        "4" => assert_eq!(tile, &Tile::Known(Digit::new(4))),
        "5" => assert_eq!(tile, &Tile::Known(Digit::new(5))),
        "6" => assert_eq!(tile, &Tile::Known(Digit::new(6))),
        "7" => assert_eq!(tile, &Tile::Known(Digit::new(7))),
        "8" => assert_eq!(tile, &Tile::Known(Digit::new(8))),
        "9" => assert_eq!(tile, &Tile::Known(Digit::new(9))),
        "." => assert!(matches!(tile, Tile::Possibilities(_))),
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
pub(crate) fn assert_tile_group_equal(group: &TileGroup, expected: &str) {
    assert_eq!(expected.len(), 9, "Expected result should have 9 digits");
    assert_eq!(group.to_test_string(), expected);
}
