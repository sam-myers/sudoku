use crate::board::Board;

pub trait Strategy {
    fn round(&self, board: Board) -> Board;
}
