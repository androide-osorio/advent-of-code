use regex::Regex;

type GameTurn = (i32, i32, i32);

#[derive(Debug)]
pub struct Game {
	pub id: u32,
	pub turns: Vec<GameTurn>,
}

impl Game {
	pub fn new(id: u32) -> Game {
		Game {
			id,
			turns: Vec::new(),
		}
	}

	pub fn add_turn(&mut self, turn: GameTurn) {
		self.turns.push(turn);
	}

	pub fn is_possible(&self, world: (i32, i32, i32)) -> bool {
		for turn in &self.turns {
			let diffs = (world.0 - turn.0, world.1 - turn.1, world.2 - turn.2);
			if diffs.0 < 0 || diffs.1 < 0 || diffs.2 < 0 {
				return false;
			}
		}
		true
	}

	pub fn calc_min_world(&self) -> (i32, i32, i32) {
		let mut min_world = (0, 0, 0);

		for turn in &self.turns {
			if turn.0 > min_world.0 {
				min_world.0 = turn.0;
			}
			if turn.1 > min_world.1 {
				min_world.1 = turn.1;
			}
			if turn.2 > min_world.2 {
				min_world.2 = turn.2;
			}
		}

		min_world
	}

	pub fn from_game_line(line: &str) -> Game {
		return game_parser::parse_game_line(line);
	}
}

mod game_parser {
	use super::*;

	pub fn parse_game_line(line: &str) -> Game {
		let game_line_regex = Regex::new(r#"^Game (\d+): (.*)$"#).unwrap();
		let captures = game_line_regex.captures(line).unwrap();

		if captures.len() == 0 {
			panic!("Invalid game line: {}", line);
		}

		let game_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
		let game_turns = captures.get(2).unwrap().as_str();

		let mut game_obj = Game::new(game_id);

		for turn in game_turns.split(";") {
			if turn.trim().is_empty() {
				continue;
			}
			let parsed_turn = parse_turn(turn);
			game_obj.add_turn(parsed_turn);
		}

		game_obj
	}

	pub fn parse_turn(turn: &str) -> GameTurn {
		let mut turn_result = (0, 0, 0);
		let turn_regex = Regex::new(r#"(\d+)\s(red|green|blue)"#).unwrap();

		if turn.len() == 0 {
			return turn_result;
		}

		for cubes in turn.split(",") {
			let captures = turn_regex.captures(cubes).unwrap();
			let num_cubes = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
			let color = captures.get(2).unwrap().as_str();

			match color {
				"red"   => turn_result.0 = num_cubes,
				"green" => turn_result.1 = num_cubes,
				"blue"  => turn_result.2 = num_cubes,
				_       => panic!("Invalid color: {}", color),
			}
		}
		turn_result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_game_line() {
		let line = "Game 1: 18 red, 8 green, 7 blue; 15 red, 4 blue, 1 green; 16 red, 5 green";
		let game = game_parser::parse_game_line(line);

		assert_eq!(game.id, 1);
		assert_eq!(game.turns[0], (18, 8, 7));
		assert_eq!(game.turns[1], (15, 1, 4));
		assert_eq!(game.turns[2], (16, 5, 0));
	}

	#[test]
	fn test_parse_game_line_empty() {
		let line = "Game 1: ; ; ";
		let game = game_parser::parse_game_line(line);

		assert_eq!(game.id, 1);
		assert_eq!(game.turns.len(), 0);
	}

	#[test]
	fn test_parse_game_line_no_turns() {
		let line = "Game 1: ";
		let game = game_parser::parse_game_line(line);

		assert_eq!(game.id, 1);
		assert_eq!(game.turns.len(), 0);
	}

	#[test]
	fn test_parse_turn() {
		let sample_turn = game_parser::parse_turn("18 red, 8 green, 7 blue");
		let empty_turn1 = game_parser::parse_turn("0 red, 0 green, 0 blue");
		let empty_turn2 = game_parser::parse_turn("");

		assert_eq!(sample_turn, (18, 8, 7));
		assert_eq!(empty_turn1, (0, 0, 0));
		assert_eq!(empty_turn2, (0, 0, 0));
	}

	#[test]
	fn test_check_game_possible() {
		let world = (15, 11, 10);
		let game1 = Game {
			id: 1,
			turns: vec![
				(14, 8, 7),
				(15, 11, 4),
				(12, 5, 9),
			],
		};

		assert_eq!(game1.is_possible(world), true);

		let game2 = Game {
			id: 2,
			turns: vec![
				(16, 8, 7),
				(15, 11, 4),
				(12, 5, 9),
			],
		};

		assert_eq!(game2.is_possible(world), false);
	}

	#[test]
	fn test_calc_min_world() {
		let game1 = Game {
			id: 1,
			turns: vec![
				(14, 8, 7),
				(15, 11, 4),
				(12, 5, 9),
			],
		};
		assert_eq!(game1.calc_min_world(), (15, 11, 9));
	}
}
