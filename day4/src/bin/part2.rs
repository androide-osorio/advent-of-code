use day4::scratchcards;

fn parse_cards_file(data: &str) -> Vec<scratchcards::ScratchCard> {
    let mut cards: Vec<scratchcards::ScratchCard> = Vec::new();

    for line in data.lines() {
        cards.push(scratchcards::ScratchCard::from_card_line(&line));
    }
    cards
}

fn main() {
    println!("Part 2!");
    let data = include_str!("../data.txt");
    let cards = parse_cards_file(data);
    let mut counts = vec![1u32; cards.len()];

    for i in 0..cards.len() {
        for j in 0..(cards[i].get_matches() as usize) {
            counts[(i + j + 1) as usize] += counts[i];
        }
    }

    println!("Total points: {}", counts.iter().sum::<u32>());
}
