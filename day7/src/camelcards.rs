use itertools::Itertools;
use std::collections::HashMap;

pub type CardValueMap = HashMap<char, u8>;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Hand {
    pub original: String,
    pub values: Vec<u8>,
}

impl Hand {
    pub fn from_str(hand: &str, card_map: &CardValueMap) -> Hand {
        let values = hand.chars().map(|c| *card_map.get(&c).unwrap()).collect();
        Hand {
            original: hand.to_string(),
            values,
        }
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

pub struct Game {
    pub hands: HashMap<String, u32>,
    pub card_map: CardValueMap,
    pub calc_hand_strength: fn(&Hand, &CardValueMap) -> HandType,
}

impl Game {
    pub fn new(
        hands: HashMap<String, u32>,
        card_map: CardValueMap,
        calc_hand_strength: fn(&Hand, &CardValueMap) -> HandType,
    ) -> Game {
        Game {
            hands,
            card_map,
            calc_hand_strength,
        }
    }
    pub fn get_winning_for_hand(&self, hand: &Hand) -> Option<&u32> {
        let str_hand = hand.original.clone();
        self.hands.get(&str_hand)
    }

    pub fn get_sorted_hands(&self) -> Vec<Hand> {
        self.hands
            .keys()
            .cloned()
            .into_iter()
            .map(|hand| Hand::from_str(&*hand, &self.card_map))
            .sorted_by(|h1, h2| {
                let h1_type = (self.calc_hand_strength)(h1, &self.card_map);
                let h2_type = (self.calc_hand_strength)(h2, &self.card_map);
                let h1_vals = h1.values.clone();
                let h2_vals = h2.values.clone();

                if h1_type == h2_type {
                    return h1_vals.cmp(&h2_vals);
                }

                return h1_type.cmp(&h2_type);
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        let card_map: CardValueMap = [('A', 1), ('B', 2), ('C', 3)].iter().cloned().collect();

        let hand = Hand::from_str("ABC", &card_map);

        assert_eq!(hand.original, "ABC");
        assert_eq!(hand.values, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_winning_for_hand() {
        let hands: HashMap<String, u32> = [("ABC".to_string(), 1), ("DEF".to_string(), 2)]
            .iter()
            .cloned()
            .collect();

        let card_map: CardValueMap = [('A', 1), ('B', 2), ('C', 3)].iter().cloned().collect();

        let game = Game::new(hands, card_map, |_, _| HandType::HighCard);

        let hand = Hand::from_str("ABC", &card_map);
        let winning = game.get_winning_for_hand(&hand);

        assert_eq!(winning, Some(&1));
    }

    #[test]
    fn test_get_sorted_hands() {
        let hands: HashMap<String, u32> = [
            ("ABC".to_string(), 1),
            ("DEF".to_string(), 2),
            ("GHI".to_string(), 3),
        ]
        .iter()
        .cloned()
        .collect();

        let card_map: CardValueMap = [('A', 1), ('B', 2), ('C', 3)].iter().cloned().collect();

        let game = Game::new(hands, card_map, |_, _| HandType::HighCard);

        let sorted_hands = game.get_sorted_hands();

        assert_eq!(sorted_hands.len(), 3);
        assert_eq!(sorted_hands[0].original, "ABC");
        assert_eq!(sorted_hands[1].original, "DEF");
        assert_eq!(sorted_hands[2].original, "GHI");
    }
}
