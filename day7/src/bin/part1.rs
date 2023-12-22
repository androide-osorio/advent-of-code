use std::collections::HashMap;

use day7::camelcards::{CardValueMap, Game, Hand, HandType};

use itertools::Itertools;

fn parse_game_data(data: &str) -> HashMap<String, u32> {
    let input: Vec<&str> = data.lines().collect();

    input
        .iter()
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.split_whitespace().collect();
            if parts.len() == 2 {
                parts[1]
                    .parse::<u32>()
                    .ok()
                    .map(|winnings| (parts[0].to_string(), winnings))
            } else {
                None
            }
        })
        .collect()
}

fn get_hand_type(hand: &Hand, _map: &CardValueMap) -> HandType {
    let counts = hand.original.chars().into_iter().counts();
    let fingerprints = counts.values().sorted().join("");

    match fingerprints.as_str() {
        "5" => HandType::FiveOfAKind,
        "14" => HandType::FourOfAKind,
        "23" => HandType::FullHouse,
        "113" => HandType::ThreeOfAKind,
        "122" => HandType::TwoPair,
        "1112" => HandType::OnePair,
        "11111" => HandType::HighCard,
        _ => panic!("Invalid card count"),
    }
}

fn get_total_winnings(game: &Game) -> u32 {
    let ordered_hands = game.get_sorted_hands();
    ordered_hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        let winnings = *game.get_winning_for_hand(hand).unwrap();
        let multiplier = i + 1;
        acc + winnings * multiplier as u32
    })
}

pub fn main() {
    println!("Part 1!");

    let card_map: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    let data: &'static str = include_str!("../data.txt");
    let parsed_input = parse_game_data(data);
    let game = Game::new(parsed_input, card_map.clone(), get_hand_type);

    let winnings = get_total_winnings(&game);
    println!("Result: {}", winnings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_winnings() {
        let input = HashMap::from([
            ("32T3K".to_string(), 765),
            ("T55J5".to_string(), 684),
            ("KK677".to_string(), 28),
            ("KTJJT".to_string(), 220),
            ("QQQJA".to_string(), 483),
        ]);

        let card_map: HashMap<char, u8> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);

        let game = Game::new(input, card_map.clone(), get_hand_type);
        assert_eq!(get_total_winnings(&game), 6440);
    }
}
