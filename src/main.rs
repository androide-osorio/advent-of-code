use clap::Parser;

pub mod puzzle2;
pub mod puzzle4;
pub mod puzzle6;
pub mod puzzle7;

/// The advent of code solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The puzzle to run
    puzzle: String,

    /// The part of the specific day to run
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() {
    let cli = Cli::parse();
    let name = cli.puzzle;
    let puzzle_part = cli.part;

    match name.as_str() {
        "puzzle2" => puzzle2::run(puzzle_part),
        "puzzle4" => puzzle4::run(puzzle_part),
        "puzzle6" => puzzle6::run(puzzle_part),
        "puzzle7" => puzzle7::run(puzzle_part),
        _ => println!("Invalid puzzle name"),
    }
}
