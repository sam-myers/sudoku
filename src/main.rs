mod board;
mod digit;
mod error;
mod helpers;
mod importer;
mod strategy;
mod tile;
mod tile_group;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::process::exit;

use crate::error::Result;
use crate::helpers::open_file;
use crate::importer::{Importer, SDKImporter};
use crate::strategy::{solve, SweepTileStrategy};

fn main() {
    let matches = App::new("sudoku")
        .version("0.0.1")
        .about("Sudoku puzzle utilities")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("solve").about("Solve a puzzle").arg(
                Arg::with_name("input")
                    .short("i")
                    .required(true)
                    .takes_value(true)
                    .help("SDK file to solve"),
            ),
        )
        .get_matches();

    if let Some(solve_matches) = matches.subcommand_matches("solve") {
        if let Err(e) = solve_command(solve_matches) {
            println!("Error: {}", e.to_string());
            exit(1);
        }
        exit(0);
    }
}

fn solve_command(matches: &ArgMatches) -> Result<()> {
    let filename = matches.value_of("input").unwrap();
    let mut file = open_file(filename)?;
    let mut board = SDKImporter.parse(&mut file)?;

    println!("Solving puzzle");
    println!("{}", board);

    board = solve(board)?;

    println!("Solved puzzle");
    println!("{}", board);
    Ok(())
}
