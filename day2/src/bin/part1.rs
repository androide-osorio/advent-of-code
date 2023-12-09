use day2::game;

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
    let world = (12, 13, 14);

    let possible_games_sum = games
        .iter()
        .filter(|game| game.is_possible(world))
        .fold(0, |sum, game| sum + game.id);

    println!("Possible games: {}", possible_games_sum);
}
