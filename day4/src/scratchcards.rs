use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
pub struct ScratchCard {
	pub id: u32,
	pub winning_numbers: HashSet<u32>,
	pub own_numbers: HashSet<u32>,
}

impl ScratchCard {
	pub fn new(id: u32) -> ScratchCard {
		ScratchCard {
			id,
			winning_numbers: HashSet::new(),
			own_numbers: HashSet::new(),
		}
	}

	pub fn get_matches(&self) -> u32 {
		let intersection = self.winning_numbers.intersection(&self.own_numbers);
		let intersection_count = intersection.count() as u32;

		intersection_count
	}

	pub fn get_points(&self) -> u32 {
		let matches = self.get_matches();

		if matches == 0 {
			return 0;
		}
		let power = 2u32.pow(matches - 1);
		power
	}

	pub fn from_card_line(line: &str) -> ScratchCard {
		return scratchcard_parser::parse_scratchcard_line(line);
	}
}

mod scratchcard_parser {
	use super::*;

	pub fn parse_scratchcard_line(line: &str) -> ScratchCard {
		let scratchcard_line_regex = Regex::new(r#"^Card\s+(\d+):\s(.*)\s\|\s(.*)$"#).unwrap();

		let captures = scratchcard_line_regex.captures(line).unwrap();
		let id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
		let winning_nums = captures.get(2).unwrap().as_str().split_whitespace();
		let own_nums = captures.get(3).unwrap().as_str().split_whitespace();
		let mut scratchcard = ScratchCard::new(id);

		for number in winning_nums {
			let number = number.parse::<u32>().unwrap();
			scratchcard.winning_numbers.insert(number);
		}

		for number in own_nums {
			let number = number.parse::<u32>().unwrap();
			scratchcard.own_numbers.insert(number);
		}

		scratchcard
	}
}