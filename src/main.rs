mod board;
mod error;
mod importer;
mod num;
mod strategy;
mod test_utils;
mod tile;

use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};
use std::path::Path;
use std::fs::File;
use std::process::exit;

use crate::error::ImportError;
use crate::importer::{Importer, SDKImporter};
use crate::strategy::{Strategy, SweepTileStrategy};

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
                ImportError => println!("Couldn't import puzzle"),
            }
            exit(1);
        }
        exit(0);
    }
}

fn solve(matches: &ArgMatches) -> Result<(), ImportError> {
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
