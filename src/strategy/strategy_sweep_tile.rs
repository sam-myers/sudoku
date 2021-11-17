use crate::board::Board;
use crate::strategy::Strategy;
use crate::tile::Tile;

pub struct SweepTileStrategy;

impl SweepTileStrategy {
    fn sweep_tile(&self, mut board: Board, x: usize, y: usize) -> Board {
        if let Tile::Known(num) = board.grid[x][y] {
            // Horizontal
            for i in 0..9 {
                board.grid[i][y] = board.grid[i][y].remove_possibility(num);
            }

            // Vertical
            for i in 0..9 {
                board.grid[x][i] = board.grid[x][i].remove_possibility(num);
            }

            // Grid
            let grid_x: usize = x / 3;
            let grid_y: usize = y / 3;
            for i in 0..3 {
                for j in 0..3 {
                    let check_x = grid_x * 3 + i;
                    let check_y = grid_y * 3 + j;

                    if check_x == x && check_y == y {
                        continue;
                    }

                    board.grid[check_x][check_y] =
                        board.grid[check_x][check_y].remove_possibility(num);
                }
            }
        }
        board
    }
}

impl Strategy for SweepTileStrategy {
    fn round(&self, mut board: Board) -> Board {
        for x in 0..9 {
            for y in 0..9 {
                board = self.sweep_tile(board, x, y);
            }
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digit::Digit;
    use crate::test_utils::get_test;
    use crate::tile::Tile;

    #[test]
    fn test_single_tile() {
        let mut b = get_test("single_tile").unwrap();
        b = SweepTileStrategy.sweep_tile(b, 0, 0);
        assert_eq!(
            b.grid[0][1],
            Tile::Possibilities([false, true, true, true, true, true, true, true, true,])
        )
    }

    #[test]
    fn test_seven_vertical() {
        let mut b = get_test("vertical_7").unwrap();
        b = SweepTileStrategy.round(b);
        assert_eq!(
            b.grid[0][8],
            Tile::Possibilities([false, false, false, false, false, false, false, true, true,])
        )
    }

    #[test]
    fn test_eight_vertical() {
        let mut b = get_test("vertical_8").unwrap();
        b = SweepTileStrategy.round(b);
        assert_eq!(b.get(0, 8), &Tile::Known(Digit::new(9)))
    }
}
