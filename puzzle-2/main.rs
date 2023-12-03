use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

pub mod game;

fn parse_game_file(file_path: &str) -> Vec<game::Game> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut games: Vec<game::Game> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let game = game::Game::from_game_line(&line);
            games.push(game);
        }
    }

    games
}

fn part_1(games: Vec<game::Game>) {
    let world = (12, 13, 14);

    let possible_games_sum = games
        .iter()
        .filter(|game| game.is_possible(world))
        .fold(0, |sum, game| sum + game.id);

    println!("Possible games: {}", possible_games_sum);
}

fn part_2(games: Vec<game::Game>) {
    let all_game_powers = games
        .iter()
        .map(|game| game.calc_min_world())
        .map(|world| world.0 * world.1 * world.2)
        .fold(0, |sum, power| sum + power);

    println!("Part 2!");
    println!("All game powers: {}", all_game_powers);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command line argument (part1 or part2)");
        return;
    }

    let file_path = "./data/games.txt";
    let games = parse_game_file(file_path);

    match args[1].as_str() {
        "part1" => part_1(games),
        "part2" => part_2(games),
        _ => println!("Invalid command line argument"),
    }
}
