use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

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
pub enum HandTypes {
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
    pub hand_type: HandTypes,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        let hand_type = Hand::classify(&cards);
        Hand {
            cards: cards,
            hand_type: hand_type,
        }
    }

    pub fn from_str(hand: &str) -> Hand {
        let mut cards: Vec<Card> = Vec::new();
        for card in hand.chars() {
            let rank = Card::from_str(&card.to_string());
            cards.push(rank);
        }
        Hand::new(cards)
    }

    pub fn get_ordered_cards(&self) -> Vec<Card> {
        let mut ordered_cards = self.cards.clone();
        ordered_cards.sort();
        ordered_cards.reverse();
        ordered_cards
    }

    fn classify(cards: &Vec<Card>) -> HandTypes {
        let mut ordered_cards = cards.clone();
        ordered_cards.sort();
        ordered_cards.reverse();

        let mut indexes: Vec<u8> = vec![0; 13];

        for card in ordered_cards.iter() {
            let rank_value = u8::from(*card) - 2;
            indexes[rank_value as usize] += 1;
        }

        let mut counts: Vec<(u8, Card)> = indexes
            .into_iter()
            .enumerate()
            .map(|(i, count)| (count as u8, Card::from(i as u8 + 2)))
            .collect();
        counts.sort();
        counts.reverse();

        let (count, _rank) = counts[0];
        return match count {
            5 => HandTypes::FiveOfAKind,
            4 => HandTypes::FourOfAKind,
            3 => {
                if counts[1].0 == 2 {
                    return HandTypes::FullHouse;
                } else {
                    return HandTypes::ThreeOfAKind;
                }
            }
            2 => {
                if counts[1].0 == 2 {
                    return HandTypes::TwoPair;
                } else {
                    return HandTypes::OnePair;
                }
            }
            1 => HandTypes::HighCard,
            _ => panic!("Invalid card count"),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
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
        self.hand_type.hash(state);
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
    fn test_hand_classify() {
        let hand = Hand::from_str("23456");
        assert_eq!(hand.hand_type, HandTypes::HighCard);

        let hand = Hand::from_str("22222");
        assert_eq!(hand.hand_type, HandTypes::FiveOfAKind);

        let hand = Hand::from_str("22223");
        assert_eq!(hand.hand_type, HandTypes::FourOfAKind);

        let hand = Hand::from_str("22233");
        assert_eq!(hand.hand_type, HandTypes::FullHouse);

        let hand = Hand::from_str("22234");
        assert_eq!(hand.hand_type, HandTypes::ThreeOfAKind);

        let hand = Hand::from_str("22334");
        assert_eq!(hand.hand_type, HandTypes::TwoPair);

        let hand = Hand::from_str("22345");
        assert_eq!(hand.hand_type, HandTypes::OnePair);
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
