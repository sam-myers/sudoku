use crate::board::Board;
use crate::error_invalid_puzzle::InvalidPuzzle;

pub trait Strategy {
    fn round(&self, board: Board) -> Result<Board, InvalidPuzzle>;
}
