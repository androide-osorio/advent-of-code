use day7::camelcards;

use std::collections::HashMap;
use camelcards::Hand;

pub enum Direction {
    Ascending,
    Descending,
}

type Game = HashMap<Hand, u32>;

fn get_ordered_hands(game: &Game, direction: Direction) -> Vec<Hand> {
    let mut ordered_hands: Vec<Hand> = game.keys().cloned().collect();
    ordered_hands.sort_by(|a, b| a.cmp(b));

    match direction {
        Direction::Ascending => (),
        Direction::Descending => ordered_hands.reverse(),
    }

    ordered_hands
}

fn get_total_winnings(game: &Game) -> u32 {
    let ordered_hands = get_ordered_hands(game, Direction::Ascending);
    ordered_hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        let winnings = game.get(hand).unwrap();
        let multiplier = i + 1;
        acc + winnings * multiplier as u32
    })
}

pub fn main() {
    println!("Part 1!");

    let data: &'static str = include_str!("../data.txt");
    let input: Vec<&str> = data.lines().collect();

    let game: HashMap<Hand, u32> = input.iter()
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.split_whitespace().collect();
            if parts.len() == 2 {
                parts[1].parse::<u32>().ok().map(|winnings| {
                    (Hand::from_str(parts[0]), winnings)
                })
            } else {
                None
            }
        })
        .collect();

    let winnings = get_total_winnings(&game);
    println!("Result: {}", winnings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_hands_asc() {
        let game = HashMap::from([
            (Hand::from_str("32T3K"), 765),
            (Hand::from_str("T55J5"), 684),
            (Hand::from_str("KK677"), 28),
            (Hand::from_str("KTJJT"), 220),
            (Hand::from_str("QQQJA"), 483),
        ]);

        let ordered_hands = get_ordered_hands(&game, Direction::Ascending);

        assert_eq!(ordered_hands[0], Hand::from_str("32T3K"));
        assert_eq!(ordered_hands[1], Hand::from_str("KTJJT"));
        assert_eq!(ordered_hands[2], Hand::from_str("KK677"));
        assert_eq!(ordered_hands[3], Hand::from_str("T55J5"));
        assert_eq!(ordered_hands[4], Hand::from_str("QQQJA"));
    }

    #[test]
    fn test_ordered_hands_desc() {
        let game = HashMap::from([
            (Hand::from_str("32T3K"), 765),
            (Hand::from_str("T55J5"), 684),
            (Hand::from_str("KK677"), 28),
            (Hand::from_str("KTJJT"), 220),
            (Hand::from_str("QQQJA"), 483),
        ]);

        let ordered_hands = get_ordered_hands(&game, Direction::Descending);

        assert_eq!(ordered_hands[0], Hand::from_str("QQQJA"));
        assert_eq!(ordered_hands[1], Hand::from_str("T55J5"));
        assert_eq!(ordered_hands[2], Hand::from_str("KK677"));
        assert_eq!(ordered_hands[3], Hand::from_str("KTJJT"));
        assert_eq!(ordered_hands[4], Hand::from_str("32T3K"));
    }

    #[test]
    fn test_total_winnings() {
        let game = HashMap::from([
            (Hand::from_str("32T3K"), 765),
            (Hand::from_str("T55J5"), 684),
            (Hand::from_str("KK677"), 28),
            (Hand::from_str("KTJJT"), 220),
            (Hand::from_str("QQQJA"), 483),
        ]);

        assert_eq!(get_total_winnings(&game), 6440);
    }
}
