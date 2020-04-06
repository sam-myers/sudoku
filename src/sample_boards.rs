use crate::board::Board;
use crate::error::Result;
use crate::num;

#[allow(dead_code)]
pub fn board_1() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::Three)?
        .given(0, 5, num::Num::Two)?
        .given(0, 6, num::Num::Five)?

        .given(1, 2, num::Num::Eight)?
        .given(1, 6, num::Num::Two)?
        .given(1, 7, num::Num::Six)?

        .given(2, 2, num::Num::Two)?
        .given(2, 4, num::Num::Three)?
        .given(2, 8, num::Num::Nine)?

        .given(3, 1, num::Num::Four)?
        .given(3, 2, num::Num::Six)?
        .given(3, 3, num::Num::Nine)?
        .given(3, 5, num::Num::Seven)?

        .given(4, 0, num::Num::Eight)?

        .given(5, 4, num::Num::Two)?
        .given(5, 5, num::Num::Six)?
        .given(5, 7, num::Num::One)?

        .given(6, 2, num::Num::Nine)?
        .given(6, 4, num::Num::One)?
        .given(6, 7, num::Num::Five)?

        .given(7, 3, num::Num::Three)?
        .given(7, 6, num::Num::Four)?
        .given(7, 7, num::Num::Seven)?

        .given(8, 1, num::Num::Three)?
        .given(8, 4, num::Num::Seven)
}

#[allow(dead_code)]
pub fn board_2() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::Eight)?
        .given(0, 1, num::Num::Two)?
//        .given(0, 2, num::Num::Seven)?
        .given(0, 3, num::Num::One)?
        .given(0, 4, num::Num::Five)?
        .given(0, 5, num::Num::Four)?
        .given(0, 6, num::Num::Three)?
        .given(0, 7, num::Num::Nine)?
        .given(0, 8, num::Num::Six)?

        .given(1, 0, num::Num::Nine)?
        .given(1, 1, num::Num::Six)?
        .given(1, 2, num::Num::Five)?
        .given(1, 3, num::Num::Three)?
        .given(1, 4, num::Num::Two)?
        .given(1, 5, num::Num::Seven)?
        .given(1, 6, num::Num::One)?
        .given(1, 7, num::Num::Four)?
        .given(1, 8, num::Num::Eight)?

        .given(2, 0, num::Num::Three)?
        .given(2, 1, num::Num::Four)?
        .given(2, 2, num::Num::One)?
        .given(2, 3, num::Num::Six)?
        .given(2, 4, num::Num::Eight)?
        .given(2, 5, num::Num::Nine)?
        .given(2, 6, num::Num::Seven)?
        .given(2, 7, num::Num::Five)?
        .given(2, 8, num::Num::Two)?


        .given(3, 0, num::Num::Five)?
        .given(3, 1, num::Num::Nine)?
        .given(3, 2, num::Num::Three)?
        .given(3, 3, num::Num::Four)?
        .given(3, 4, num::Num::Six)?
        .given(3, 5, num::Num::Eight)?
        .given(3, 6, num::Num::Two)?
        .given(3, 7, num::Num::Seven)?
        .given(3, 8, num::Num::One)?

        .given(4, 0, num::Num::Four)?
        .given(4, 1, num::Num::Seven)?
        .given(4, 2, num::Num::Two)?
        .given(4, 3, num::Num::Five)?
        .given(4, 4, num::Num::One)?
        .given(4, 5, num::Num::Three)?
        .given(4, 6, num::Num::Six)?
        .given(4, 7, num::Num::Eight)?
        .given(4, 8, num::Num::Nine)?

        .given(5, 0, num::Num::Six)?
        .given(5, 1, num::Num::One)?
        .given(5, 2, num::Num::Eight)?
        .given(5, 3, num::Num::Nine)?
        .given(5, 4, num::Num::Seven)?
        .given(5, 5, num::Num::Two)?
        .given(5, 6, num::Num::Four)?
        .given(5, 7, num::Num::Three)?
        .given(5, 8, num::Num::Five)?


        .given(6, 0, num::Num::Seven)?
        .given(6, 1, num::Num::Eight)?
        .given(6, 2, num::Num::Six)?
        .given(6, 3, num::Num::Two)?
        .given(6, 4, num::Num::Three)?
        .given(6, 5, num::Num::Five)?
        .given(6, 6, num::Num::Nine)?
        .given(6, 7, num::Num::One)?
        .given(6, 8, num::Num::Four)?

        .given(7, 0, num::Num::One)?
        .given(7, 1, num::Num::Five)?
        .given(7, 2, num::Num::Four)?
        .given(7, 3, num::Num::Seven)?
        .given(7, 4, num::Num::Nine)?
        .given(7, 5, num::Num::Six)?
        .given(7, 6, num::Num::Eight)?
        .given(7, 7, num::Num::Two)?
        .given(7, 8, num::Num::Three)?

        .given(8, 0, num::Num::Two)?
        .given(8, 1, num::Num::Three)?
        .given(8, 2, num::Num::Nine)?
        .given(8, 3, num::Num::Eight)?
        .given(8, 4, num::Num::Four)?
        .given(8, 5, num::Num::One)?
        .given(8, 6, num::Num::Five)?
        .given(8, 7, num::Num::Six)?
        .given(8, 8, num::Num::Seven)
}

#[allow(dead_code)]
fn invalid_1() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::Three)?
        .given(0, 8, num::Num::Three)
}

#[allow(dead_code)]
fn invalid_2() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::Three)?
        .given(7, 0, num::Num::Three)
}

#[allow(dead_code)]
fn invalid_3() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::Three)?
        .given(2, 2, num::Num::Three)
}

#[allow(dead_code)]
fn invalid_vertical_1() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::One)?
//        .given(1, 0, num::Num::Three)?
        .given(2, 0, num::Num::Three)?
        .given(3, 0, num::Num::One)?
        .given(4, 0, num::Num::Five)?
        .given(5, 0, num::Num::Six)?
        .given(6, 0, num::Num::Seven)?
        .given(7, 0, num::Num::Eight)?
        .given(8, 0, num::Num::Nine)
}

#[allow(dead_code)]
fn invalid_horizontal_1() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::One)?
//        .given(0, 1, num::Num::Three)?
        .given(0, 2, num::Num::Three)?
        .given(0, 3, num::Num::Four)?
        .given(0, 4, num::Num::Five)?
        .given(0, 5, num::Num::Six)?
        .given(0, 6, num::Num::Seven)?
        .given(0, 7, num::Num::One)?
        .given(0, 8, num::Num::Nine)
}

#[allow(dead_code)]
fn invalid_grid_1() -> Result<Board> {
    Board::new()
        .given(0, 0, num::Num::One)?
//        .given(0, 1, num::Num::Three)?
        .given(0, 2, num::Num::Three)?
        .given(1, 0, num::Num::Four)?
        .given(1, 1, num::Num::Five)?
        .given(1, 2, num::Num::Six)?
        .given(2, 0, num::Num::One)?
        .given(2, 1, num::Num::Eight)?
        .given(2, 2, num::Num::Nine)
}

#[allow(dead_code)]
pub fn invalid_grid_2() -> Result<Board> {
    Board::new()
        .given(6, 6, num::Num::One)?
//        .given(6, 7, num::Num::Three)?
        .given(6, 8, num::Num::Three)?
        .given(7, 6, num::Num::Four)?
        .given(7, 7, num::Num::One)?
        .given(7, 8, num::Num::Six)?
        .given(8, 6, num::Num::Seven)?
        .given(8, 7, num::Num::Eight)?
        .given(8, 8, num::Num::Nine)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_board_1() {
        assert!(board_1().is_ok());
    }

    #[test]
    fn test_board_2() {
        assert!(board_2().is_ok());
    }

    #[test]
    fn test_invalid_1() {
        assert!(invalid_1().is_err());
    }

    #[test]
    fn test_invalid_2() {
        assert!(invalid_2().is_err());
    }

    #[test]
    fn test_invalid_3() {
        assert!(invalid_3().is_err());
    }

    #[test]
    fn test_vertical_1() {
        assert!(invalid_vertical_1().is_err());
    }

    #[test]
    fn test_invalid_horizontal_1() {
        assert!(invalid_vertical_1().is_err());
    }

    #[test]
    fn test_invalid_grid_1() {
        assert!(invalid_grid_1().is_err());
    }

    #[test]
    fn test_invalid_grid_2() {
        assert!(invalid_grid_2().is_err());
    }
}
