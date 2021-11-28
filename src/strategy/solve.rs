use crate::board::Board;
use crate::error::{Result, SudokuError};
use crate::strategy::Strategy;
use crate::SweepTileStrategy;

pub fn solve(mut board: Board) -> Result<Board> {
    let mut pre_round_board: Board;

    while !board.is_done() {
        pre_round_board = board;

        board = board.apply_strategy(SweepTileStrategy::apply)?;

        if pre_round_board == board {
            return Err(SudokuError::UnsolvablePuzzle);
        }
    }
    Ok(board)
}

#[cfg(test)]
mod tests {
    use crate::error::SudokuError;
    use crate::helpers::get_test;
    use crate::strategy::solve;

    #[test]
    fn test_solvable_puzzle() {
        let b = get_test("solvable_1").unwrap();
        assert!(solve(b).is_ok())
    }

    #[test]
    fn test_unsolvable_puzzle() {
        let b = get_test("single_tile").unwrap();
        assert!(matches!(solve(b), Err(SudokuError::UnsolvablePuzzle)));
    }
}
