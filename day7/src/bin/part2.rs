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

fn get_hand_type(hand: &Hand, map: &CardValueMap) -> HandType {
    let mut card_counts = hand.original.chars().into_iter().counts();
    let num_jokers = *card_counts.get(&'J').unwrap_or(&0);

    // dbg!(&card_counts);

    if num_jokers == 5 {
        return HandType::FiveOfAKind;
    }

    if num_jokers > 0 {
        // get the highest card value that is not a joker
        let other_cards = card_counts
            .keys()
            .filter(|&c| *c != 'J')
            .collect::<Vec<&char>>();
        let highest_card = other_cards.iter().max_by_key(|&&c| map.get(&c)).unwrap();

        // add the number of jokers to the count of the highest card
        let new_count = card_counts.get(highest_card).unwrap() + num_jokers;
        card_counts.insert(**highest_card, new_count);
        card_counts.remove(&'J');

        // dbg!(&card_counts);
    }

    let fingerprint = card_counts.values().sorted().join("");

    match fingerprint.as_str() {
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

    ordered_hands
        .iter()
        .enumerate()
        .inspect(|(i, hand)| {
            let winnings = *game.get_winning_for_hand(hand).unwrap();
            println!(
                "{}: {:?} : {} -> {}",
                i + 1,
                hand.original,
                winnings,
                winnings * (i + 1) as u32
            );
        })
        .fold(0, |acc, (i, hand)| {
            let winnings = *game.get_winning_for_hand(hand).unwrap();
            let multiplier = i + 1;
            acc + winnings * multiplier as u32
        })
}

pub fn main() {
    println!("Part 2!");

    let card_map: HashMap<char, u8> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
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
            ('J', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);

        let game = Game::new(input, card_map.clone(), get_hand_type);
        assert_eq!(get_total_winnings(&game), 5905);
    }
}
