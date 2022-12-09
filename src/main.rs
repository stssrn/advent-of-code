use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;
use pretty_env_logger;
mod day;

/// An annual set of Christmas-themed computer programming challenges that follow an Advent calendar.
#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code 2021")]
struct Opt {
    /// Calendar day
    day: u8,
    /// Puzzle number (1 or 2)
    puzzle: u8,
    /// Puzzle input
    input_file: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opt = Opt::from_args();
    let puzzle = opt.puzzle;
    let mut reader = File::open(&opt.input_file)?;
    match &opt.day {
        1 if puzzle == 1 => day::I::one(&mut reader),
        1 if puzzle == 2 => day::I::two(&mut reader),
        2 if puzzle == 1 => day::II::one(&mut reader),
        2 if puzzle == 2 => day::II::two(&mut reader),
        3 if puzzle == 1 => day::III::one(&mut reader),
        3 if puzzle == 2 => day::III::two(&mut reader),
        4 if puzzle == 1 => day::IV::one(&mut reader),
        _ => eprintln!("invalid!")
    }
    Ok(())
}
