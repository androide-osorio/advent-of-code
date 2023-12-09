pub mod scratchcards;

fn parse_cards_file(data: &str) -> Vec<scratchcards::ScratchCard> {
    let mut cards: Vec<scratchcards::ScratchCard> = Vec::new();

    for line in data.lines() {
        cards.push(scratchcards::ScratchCard::from_card_line(&line));
    }
    cards
}

fn main() {
    println!("Part 1!");
    let data = include_str!("../data.txt");
    let scratchcards = parse_cards_file(data);
    let total_points = scratchcards
        .iter()
        .map(|card| card.get_points())
        .fold(0, |sum, points| sum + points);

    println!("Total points: {}", total_points);
}
