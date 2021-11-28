use std::fmt;

use crate::digit::Digit;
use crate::error::{Result, SudokuError};
use crate::tile::Tile;
use crate::tile_group::{TileGroup, TileGroupLocation};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Board {
    pub grid: [[Tile; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
                Board::new_row(),
            ],
        }
    }

    pub fn given(mut self, x: usize, y: usize, num: Digit) -> Result<Self> {
        let tile = Tile::Known(num);

        for i in 0..9 {
            // Horizontal
            if self.grid[i][y] == tile {
                return Err(SudokuError::InvalidPuzzle);
            }

            // Vertical
            if self.grid[x][i] == tile {
                return Err(SudokuError::InvalidPuzzle);
            }
        }

        let house_x: usize = x / 3;
        let house_y: usize = y / 3;

        for i in 0..3 {
            for j in 0..3 {
                let check_x = house_x * 3 + i;
                let check_y = house_y * 3 + j;

                if check_x == x && check_y == y {
                    continue;
                }

                // House
                if self.grid[check_x][check_y] == tile {
                    return Err(SudokuError::InvalidPuzzle);
                }
            }
        }

        self.grid[x][y] = tile;
        Ok(self)
    }

    pub fn is_done(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if let Tile::Possibilities(_) = self.grid[i][j] {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_tile_group(&self, location: TileGroupLocation) -> TileGroup {
        let mut tiles: [Tile; 9] = [Tile::Known(Digit::new(1)); 9];
        match location {
            TileGroupLocation::Column(col) => {
                for x in 0..9 {
                    tiles[x] = self.grid[x][col as usize];
                }
            }
            TileGroupLocation::Row(row) => {
                for y in 0..9 {
                    tiles[y] = self.grid[row as usize][y];
                }
            }
            TileGroupLocation::House(house_x, house_y) => {
                let mut index = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        let grid_x: usize = (house_x * 3 + x) as usize;
                        let grid_y: usize = (house_y * 3 + y) as usize;
                        tiles[index] = self.grid[grid_x][grid_y];
                        index += 1;
                    }
                }
            }
        }
        TileGroup::new(location, tiles).unwrap()
    }

    pub fn save_tile_group(mut self, group: TileGroup) -> Self {
        match group.location {
            TileGroupLocation::Column(col) => {
                for x in 0..9 {
                    self.grid[x][col as usize] = group.tiles[x];
                }
            }
            TileGroupLocation::Row(row) => {
                for y in 0..9 {
                    self.grid[row as usize][y] = group.tiles[y];
                }
            }
            TileGroupLocation::House(house_x, house_y) => {
                let mut index = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        let grid_x: usize = (house_x * 3 + x) as usize;
                        let grid_y: usize = (house_y * 3 + y) as usize;
                        self.grid[grid_x][grid_y] = group.tiles[index];
                        index += 1;
                    }
                }
            }
        }
        self
    }

    pub fn apply_strategy(mut self, strategy: fn(TileGroup) -> Result<TileGroup>) -> Result<Self> {
        // Columns
        for col in 0..9 {
            self = self.save_tile_group(strategy(
                self.get_tile_group(TileGroupLocation::Column(col)),
            )?);
        }

        // Rows
        for row in 0..9 {
            self =
                self.save_tile_group(strategy(self.get_tile_group(TileGroupLocation::Row(row)))?);
        }

        // Houses
        for x in 0..3 {
            for y in 0..3 {
                self = self.save_tile_group(strategy(
                    self.get_tile_group(TileGroupLocation::House(x, y)),
                )?);
            }
        }

        Ok(self)
    }

    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize) -> &Tile {
        &self.grid[x][y]
    }

    fn new_row() -> [Tile; 9] {
        [
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
            Tile::new_blank(),
        ]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "╔══════════╦══════════╦══════════╗
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
╠══════════╬══════════╬══════════╣
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
╠══════════╬══════════╬══════════╣
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
║ {}  {}  {}  ║ {}  {}  {}  ║ {}  {}  {}  ║
╚══════════╩══════════╩══════════╝
",
            &self.grid[0][0],
            &self.grid[0][1],
            &self.grid[0][2],
            &self.grid[0][3],
            &self.grid[0][4],
            &self.grid[0][5],
            &self.grid[0][6],
            &self.grid[0][7],
            &self.grid[0][8],
            &self.grid[1][0],
            &self.grid[1][1],
            &self.grid[1][2],
            &self.grid[1][3],
            &self.grid[1][4],
            &self.grid[1][5],
            &self.grid[1][6],
            &self.grid[1][7],
            &self.grid[1][8],
            &self.grid[2][0],
            &self.grid[2][1],
            &self.grid[2][2],
            &self.grid[2][3],
            &self.grid[2][4],
            &self.grid[2][5],
            &self.grid[2][6],
            &self.grid[2][7],
            &self.grid[2][8],
            &self.grid[3][0],
            &self.grid[3][1],
            &self.grid[3][2],
            &self.grid[3][3],
            &self.grid[3][4],
            &self.grid[3][5],
            &self.grid[3][6],
            &self.grid[3][7],
            &self.grid[3][8],
            &self.grid[4][0],
            &self.grid[4][1],
            &self.grid[4][2],
            &self.grid[4][3],
            &self.grid[4][4],
            &self.grid[4][5],
            &self.grid[4][6],
            &self.grid[4][7],
            &self.grid[4][8],
            &self.grid[5][0],
            &self.grid[5][1],
            &self.grid[5][2],
            &self.grid[5][3],
            &self.grid[5][4],
            &self.grid[5][5],
            &self.grid[5][6],
            &self.grid[5][7],
            &self.grid[5][8],
            &self.grid[6][0],
            &self.grid[6][1],
            &self.grid[6][2],
            &self.grid[6][3],
            &self.grid[6][4],
            &self.grid[6][5],
            &self.grid[6][6],
            &self.grid[6][7],
            &self.grid[6][8],
            &self.grid[7][0],
            &self.grid[7][1],
            &self.grid[7][2],
            &self.grid[7][3],
            &self.grid[7][4],
            &self.grid[7][5],
            &self.grid[7][6],
            &self.grid[7][7],
            &self.grid[7][8],
            &self.grid[8][0],
            &self.grid[8][1],
            &self.grid[8][2],
            &self.grid[8][3],
            &self.grid[8][4],
            &self.grid[8][5],
            &self.grid[8][6],
            &self.grid[8][7],
            &self.grid[8][8],
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::error::SudokuError;
    use crate::helpers::{assert_tile_group_equal, get_test};
    use crate::tile::Tile;
    use crate::tile_group::TileGroupLocation;

    #[test]
    fn test_valid_1() {
        assert!(get_test("single_tile").is_ok());
    }

    #[test]
    fn test_valid_2() {
        assert!(get_test("vertical_7").is_ok());
    }

    #[test]
    fn test_valid_3() {
        assert!(get_test("vertical_8").is_ok());
    }

    #[test]
    fn test_invalid_1() {
        let b = get_test("invalid_1");
        assert!(matches!(b, Err(SudokuError::InvalidPuzzle)));
    }

    #[test]
    fn test_invalid_2() {
        let b = get_test("invalid_2");
        assert!(matches!(b, Err(SudokuError::InvalidPuzzle)));
    }

    #[test]
    fn test_invalid_3() {
        let b = get_test("invalid_3");
        assert!(matches!(b, Err(SudokuError::InvalidPuzzle)));
    }

    #[test]
    fn test_get_tile_group_column() {
        let board = get_test("solvable_1").unwrap();
        let tile_group = board.get_tile_group(TileGroupLocation::Column(0));

        assert_tile_group_equal(&tile_group, "5.49.23..");
    }

    #[test]
    fn test_get_tile_group_row() {
        let board = get_test("solvable_1").unwrap();
        let tile_group = board.get_tile_group(TileGroupLocation::Row(1));

        assert_tile_group_equal(&tile_group, ".6273..98");
    }

    #[test]
    fn test_get_tile_group_house() {
        let board = get_test("solvable_1").unwrap();
        let tile_group = board.get_tile_group(TileGroupLocation::House(0, 0));

        assert_tile_group_equal(&tile_group, "5...624.9");
    }

    #[test]
    fn test_save_tile_group_column() {
        let initial_board = get_test("solvable_1").unwrap();
        let mut initial_tile_group = initial_board.get_tile_group(TileGroupLocation::Column(0));

        initial_tile_group.tiles[1] = Tile::new_known(1);
        let updated_board = initial_board.save_tile_group(initial_tile_group);

        let updated_column = updated_board.get_tile_group(TileGroupLocation::Column(0));
        assert_tile_group_equal(&updated_column, "5149.23..");

        let updated_row = updated_board.get_tile_group(TileGroupLocation::Row(1));
        assert_tile_group_equal(&updated_row, "16273..98");

        let updated_house = updated_board.get_tile_group(TileGroupLocation::House(0, 0));
        assert_tile_group_equal(&updated_house, "5..1624.9");
    }

    #[test]
    fn test_save_tile_group_row() {
        let initial_board = get_test("solvable_1").unwrap();
        let mut initial_tile_group = initial_board.get_tile_group(TileGroupLocation::Row(8));

        initial_tile_group.tiles[7] = Tile::new_known(5);
        let updated_board = initial_board.save_tile_group(initial_tile_group);

        let updated_column = updated_board.get_tile_group(TileGroupLocation::Column(7));
        assert_tile_group_equal(&updated_column, "693.1...5");

        let updated_row = updated_board.get_tile_group(TileGroupLocation::Row(8));
        assert_tile_group_equal(&updated_row, "..1...256");

        let updated_house = updated_board.get_tile_group(TileGroupLocation::House(2, 2));
        assert_tile_group_equal(&updated_house, "8.....256");
    }

    #[test]
    fn test_save_tile_group_house() {
        let initial_board = get_test("solvable_1").unwrap();
        let mut initial_tile_group = initial_board.get_tile_group(TileGroupLocation::House(1, 1));

        initial_tile_group.tiles[0] = Tile::new_known(5);
        let updated_board = initial_board.save_tile_group(initial_tile_group);

        let updated_column = updated_board.get_tile_group(TileGroupLocation::Column(3));
        assert_tile_group_equal(&updated_column, "276531...");

        let updated_row = updated_board.get_tile_group(TileGroupLocation::Row(3));
        assert_tile_group_equal(&updated_row, "9..5..4.7");

        let updated_house = updated_board.get_tile_group(TileGroupLocation::House(1, 1));
        assert_tile_group_equal(&updated_house, "5..34.1..");
    }
}
