use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Card(u8);

impl Card {
    pub fn from(value: u8) -> Card {
        if value < 2 || value > 14 {
            panic!("Invalid rank value");
        }
        Card(value)
    }

    pub fn from_str(value: &str) -> Card {
        let rank = match value {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "T" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => panic!("Invalid rank value"),
        };
        Card(rank)
    }
}

impl From<Card> for u8 {
    fn from(rank: Card) -> u8 {
        rank.0
    }
}

impl From<Card> for usize {
    fn from(rank: Card) -> usize {
        rank.0 as usize
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand {
            cards: cards,
        }
    }

    pub fn from_str(hand: &str) -> Hand {
        let cards = hand
            .chars()
            .map(|char| Card::from_str(&char.to_string()))
            .collect();

        Hand::new(cards)
    }

    pub fn get_ordered_cards(&self) -> Vec<Card> {
        self.cards.clone().into_iter().sorted().rev().collect()
    }

    pub fn get_score(&self) -> HandType {
        let cards = self.cards.clone();
        let counts = cards.into_iter().counts();
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
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let type1 = self.get_score();
        let type2 = other.get_score();
        let hand_type_cmp = type1.cmp(&type2);
        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }

        let self_cards = self.cards.clone();
        let other_cards = other.cards.clone();

        for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
            let card_cmp = self_card.cmp(&other_card);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }
        Ordering::Equal
    }
}

impl Hash for Hand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for card in &self.cards {
            card.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        let hand = Hand::from_str("23456");
        assert_eq!(
            hand.cards,
            vec![
                Card::from(2),
                Card::from(3),
                Card::from(4),
                Card::from(5),
                Card::from(6)
            ]
        );
    }

    #[test]
    fn test_hand_get_score() {
        let hand = Hand::from_str("23456");
        assert_eq!(hand.get_score(), HandType::HighCard);

        let hand = Hand::from_str("22222");
        assert_eq!(hand.get_score(), HandType::FiveOfAKind);

        let hand = Hand::from_str("22223");
        assert_eq!(hand.get_score(), HandType::FourOfAKind);

        let hand = Hand::from_str("22233");
        assert_eq!(hand.get_score(), HandType::FullHouse);

        let hand = Hand::from_str("22234");
        assert_eq!(hand.get_score(), HandType::ThreeOfAKind);

        let hand = Hand::from_str("22334");
        assert_eq!(hand.get_score(), HandType::TwoPair);

        let hand = Hand::from_str("22345");
        assert_eq!(hand.get_score(), HandType::OnePair);
    }

    #[test]
    fn test_hands_ordering() {
        let hand1 = Hand::from_str("23456");
        let hand2 = Hand::from_str("23456");
        assert_eq!(hand1, hand2);

        let hand1 = Hand::from_str("AAAAA");
        let hand2 = Hand::from_str("TTT98");
        assert!(hand1 > hand2);

        let hand1 = Hand::from_str("AAAAA");
        let hand2 = Hand::from_str("TTTTT");
        assert!(hand1 > hand2);

        let hand1 = Hand::from_str("33332");
        let hand2 = Hand::from_str("2AAAA");
        assert!(hand1 > hand2);

        let hand1 = Hand::from_str("77788");
        let hand2 = Hand::from_str("77888");
        assert!(hand1 < hand2);
    }
}
