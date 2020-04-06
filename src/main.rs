mod board;
mod error_import;
mod error_invalid_puzzle;
mod importer;
mod importer_sdk;
mod num;
mod sample_boards;
mod tile;

fn main() {
    let mut b = sample_boards::board_2().unwrap();
    println!("{}", b);
    b.solve();
    println!("{}", b);
    println!("sweeps required: {}", b.sweeps());
}
