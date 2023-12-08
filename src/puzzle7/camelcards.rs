use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Rank {
	Two = 2,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	T,
	J,
	Q,
	K,
	A,
}

impl Rank {
	pub fn from(value: usize) -> Rank {
		match value {
			2 => Rank::Two,
			3 => Rank::Three,
			4 => Rank::Four,
			5 => Rank::Five,
			6 => Rank::Six,
			7 => Rank::Seven,
			8 => Rank::Eight,
			9 => Rank::Nine,
			10 => Rank::T,
			11 => Rank::J,
			12 => Rank::Q,
			13 => Rank::K,
			14 => Rank::A,
			_ => panic!("Invalid rank value"),
		}
	}

	pub fn from_str(value: &str) -> Rank {
		match value {
			"2" => Rank::Two,
			"3" => Rank::Three,
			"4" => Rank::Four,
			"5" => Rank::Five,
			"6" => Rank::Six,
			"7" => Rank::Seven,
			"8" => Rank::Eight,
			"9" => Rank::Nine,
			"T" => Rank::T,
			"J" => Rank::J,
			"Q" => Rank::Q,
			"K" => Rank::K,
			"A" => Rank::A,
			_ => panic!("Invalid rank value"),
		}
	}
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandTypes {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub struct Hand {
	pub cards: Vec<Rank>,
	pub hand_type: HandTypes,
}

impl Hand {
	pub fn new(cards: Vec<Rank>) -> Hand {
		let hand_type = Hand::classify(&cards);
		Hand {
			cards: cards,
			hand_type: hand_type,
		}
	}

	pub fn from_str(hand: &str) -> Hand {
		let mut cards: Vec<Rank> = Vec::new();
		for card in hand.chars() {
			let rank = Rank::from_str(&card.to_string());
			cards.push(rank);
		}
		Hand::new(cards)
	}

	pub fn get_ordered_cards(&self) -> Vec<Rank> {
		let mut ordered_cards = self.cards.clone();
		ordered_cards.sort();
		ordered_cards.reverse();
		ordered_cards
	}

	fn classify(cards: &Vec<Rank>) -> HandTypes {
		let mut ordered_cards = cards.clone();
		ordered_cards.sort();
		ordered_cards.reverse();

		let mut indexes: Vec<u8> = vec![0; 13];
		for card in ordered_cards.iter() {
			indexes[*card as usize] += 1;
		}

		let mut counts: Vec<(u8, Rank)> = indexes
			.into_iter()
			.enumerate()
			.map(|(i, count)| (count, Rank::from(i + 2)))
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
		}
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rank_from() {
		assert_eq!(Rank::from(2), Rank::Two);
		assert_eq!(Rank::from(3), Rank::Three);
		assert_eq!(Rank::from(4), Rank::Four);
		assert_eq!(Rank::from(5), Rank::Five);
		assert_eq!(Rank::from(6), Rank::Six);
		assert_eq!(Rank::from(7), Rank::Seven);
		assert_eq!(Rank::from(8), Rank::Eight);
		assert_eq!(Rank::from(9), Rank::Nine);
		assert_eq!(Rank::from(10), Rank::T);
		assert_eq!(Rank::from(11), Rank::J);
		assert_eq!(Rank::from(12), Rank::Q);
		assert_eq!(Rank::from(13), Rank::K);
		assert_eq!(Rank::from(14), Rank::A);
	}

	#[test]
	fn test_rank_from_str() {
		assert_eq!(Rank::from_str("2"), Rank::Two);
		assert_eq!(Rank::from_str("3"), Rank::Three);
		assert_eq!(Rank::from_str("4"), Rank::Four);
		assert_eq!(Rank::from_str("5"), Rank::Five);
		assert_eq!(Rank::from_str("6"), Rank::Six);
		assert_eq!(Rank::from_str("7"), Rank::Seven);
		assert_eq!(Rank::from_str("8"), Rank::Eight);
		assert_eq!(Rank::from_str("9"), Rank::Nine);
		assert_eq!(Rank::from_str("T"), Rank::T);
		assert_eq!(Rank::from_str("J"), Rank::J);
		assert_eq!(Rank::from_str("Q"), Rank::Q);
		assert_eq!(Rank::from_str("K"), Rank::K);
		assert_eq!(Rank::from_str("A"), Rank::A);
	}

	#[test]
	fn test_hand_from_str() {
		let hand = Hand::from_str("23456");
		assert_eq!(hand.cards, vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six]);
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