use clap::Parser;

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

    println!("Puzzle to run: {name}");
    println!("Value for part: {}", puzzle_part);
    println!("Hello, world!");
}
