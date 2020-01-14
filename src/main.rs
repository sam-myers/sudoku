mod board;
mod num;
mod tile;

fn main() {
    println!("{}", sample_board_1());
}

fn sample_board_1() -> board::Board {
    board::Board::new()
        .given(0, 0, num::Num::Three)
        .given(0, 5, num::Num::Two)
        .given(0, 6, num::Num::Five)

        .given(1, 2, num::Num::Eight)
        .given(1, 6, num::Num::Two)
        .given(1, 7, num::Num::Six)

        .given(2, 2, num::Num::Two)
        .given(2, 4, num::Num::Three)
        .given(2, 8, num::Num::Nine)

        .given(3, 1, num::Num::Four)
        .given(3, 2, num::Num::Six)
        .given(3, 3, num::Num::Nine)
        .given(3, 5, num::Num::Seven)

        .given(4, 0, num::Num::Eight)

        .given(5, 4, num::Num::Two)
        .given(5, 5, num::Num::Six)
        .given(5, 7, num::Num::One)

        .given(6, 2, num::Num::Nine)
        .given(6, 4, num::Num::One)
        .given(6, 7, num::Num::Five)

        .given(7, 3, num::Num::Three)
        .given(7, 6, num::Num::Four)
        .given(7, 7, num::Num::Seven)

        .given(8, 1, num::Num::Three)
        .given(8, 4, num::Num::Seven)

}
