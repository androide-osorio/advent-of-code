use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_cards_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut cards: Vec<String> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
            cards.push(line);
        }
    }
    cards
}

fn part_1(scratchcards: Vec<String>) {
    println!("Part 1!");
}

fn part_2(scratchcards: Vec<String>) {
    println!("Part 2!");
}

pub fn run(part: u8) {
    let mod_path = file!();
    let current_dir = Path::new(mod_path).parent().unwrap();
    let file_path = current_dir.join("data.txt");
    let scratchcards = parse_cards_file(file_path.as_os_str().to_str().unwrap());

    match part {
        1 => part_1(scratchcards),
        2 => part_2(scratchcards),
        _ => println!("Invalid command line argument"),
    }
}
