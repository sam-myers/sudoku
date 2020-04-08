mod board;
mod error_import;
mod error_invalid_puzzle;
mod importer;
mod importer_sdk;
mod num;
mod sample_boards;
mod strategy;
mod strategy_sweep_tile;
mod tile;

use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};
use std::path::Path;
use std::fs::File;
use std::process::exit;

use crate::importer::Importer;
use crate::importer_sdk::SDKImporter;
use crate::strategy::Strategy;
use crate::strategy_sweep_tile::SweepTileStrategy;

fn main() {

    let matches = App::new("sudoku")
        .version("0.0.1")
        .about("Sudoku puzzle utilities")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("solve")
            .about("Solve a puzzle")
            .arg(Arg::with_name("input")
                .short("i")
                .required(true)
                .takes_value(true)
                .help("SDK file to solve")))
        .get_matches();

    if let Some(solve_matches) = matches.subcommand_matches("solve") {
        if let Err(e) = solve(solve_matches) {
            match e {
                error_import::ImportError => println!("Couldn't import puzzle"),
            }
            exit(1);
        }
        exit(0);
    }
}

fn solve(matches: &ArgMatches) -> Result<(), error_import::ImportError> {
    let filename = matches.value_of("input").unwrap();
    let path = Path::new(filename);
    let mut file = File::open(&path)?;
    let mut b = SDKImporter.parse(&mut file)?;

    println!("{}", b);
    while !b.is_done() {
        b = SweepTileStrategy.round(b);
    }
    println!("{}", b);
    Ok(())
}
