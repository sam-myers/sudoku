mod board;
mod error_import;
mod error_invalid_puzzle;
mod importer;
mod importer_sdk;
mod num;
mod sample_boards;
mod tile;

use clap::{Arg, App, SubCommand};
use std::path::Path;
use std::fs::File;
use crate::importer::Importer;
use crate::importer_sdk::SDKImporter;

fn main() {

    let matches = App::new("sudoku")
        .version("0.0.1")
        .about("Sudoku puzzle utilities")
        .subcommand(SubCommand::with_name("solve")
            .about("Solve a puzzle")
            .arg(Arg::with_name("input")
                .short("i")
                .required(true)
                .takes_value(true)
                .help("SDK file to solve")))
        .get_matches();

    if let Some(solve_matches) = matches.subcommand_matches("solve") {
        let filename = solve_matches.value_of("input").unwrap();
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => println!("Couldn't open file {}: {}", display, why.to_string()),
            Ok(mut file)  => {
                let i = Box::new(SDKImporter);
                let mut b = i.parse(&mut file).expect("trouble importing");
                println!("{}", b);
                b.solve();
                println!("{}", b);
            },
        };
    }
}
