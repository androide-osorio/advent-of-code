use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub mod scratchcards;

fn parse_cards_file(file_path: &str) -> Vec<scratchcards::ScratchCard> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut cards: Vec<scratchcards::ScratchCard> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            cards.push(scratchcards::ScratchCard::from_card_line(&line));
        }
    }
    cards
}

fn part_1(scratchcards: Vec<scratchcards::ScratchCard>) {
    println!("Part 1!");
    let total_points = scratchcards
        .iter()
        .map(|card| card.get_points())
        .fold(0, |sum, points| sum + points);

    println!("Total points: {}", total_points);
}

fn part_2(scratchcards: Vec<scratchcards::ScratchCard>) {
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
