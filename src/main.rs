mod board;
mod num;
mod sample_boards;
mod tile;

fn main() {
    let mut b = sample_boards::board_1();
    println!("{}", b);
    b.solve();
    println!("{}", b);
    println!("sweeps required: {}", b.sweeps());
}
