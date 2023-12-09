pub mod game;

fn parse_game_file(game_data: &str) -> Vec<game::Game> {
    let mut games: Vec<game::Game> = Vec::new();

    for line in game_data.lines() {
        let game = game::Game::from_game_line(&line);
        games.push(game);
    }

    games
}

fn main() {
		let data = include_str!("../data.txt");
    let games = parse_game_file(data);

    let all_game_powers = games
        .iter()
        .map(|game| game.calc_min_world())
        .map(|world| world.0 * world.1 * world.2)
        .fold(0, |sum, power| sum + power);

    println!("Part 2!");
    println!("All game powers: {}", all_game_powers);
}