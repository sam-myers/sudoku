use crate::board::Board;
use crate::error_invalid_puzzle::InvalidPuzzle;
use crate::strategy::Strategy;
use crate::tile::Tile;

pub struct SweepTileStrategy;

impl SweepTileStrategy {
    fn sweep_tile(&self, mut board: Board, x: usize, y: usize) -> Board {
        if let Tile::Known(num) = board.grid[x][y] {
            // Horizontal
            for i in 0..9 {
                board.grid[i][y] = board.grid[i][y].remove_possibility(&num);
            }

            // Vertical
            for i in 0..9 {
                board.grid[x][i] = board.grid[x][i].remove_possibility(&num);
            }

            // Grid
            let grid_x: usize = x / 3;
            let grid_y: usize = y / 3;
            for i in 0..3 {
                for j in 0..3 {
                    let check_x = grid_x*3 + i;
                    let check_y = grid_y*3 + j;

                    if check_x == x && check_y == y {
                        continue;
                    }

                    board.grid[check_x][check_y] = board.grid[check_x][check_y].remove_possibility(&num);
                }
            }
        }
        board
    }
}

impl Strategy for SweepTileStrategy {
    fn round(&self, mut board: Board) -> Result<Board, InvalidPuzzle> {
        for x in 0..9 {
            for y in 0..9 {
                board = self.sweep_tile(board, x, y);
            }
        }
        Ok(board)
    }
}
