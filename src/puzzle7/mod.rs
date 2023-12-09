pub mod camelcards;

pub mod part1;

pub fn run(part: u8) {
    println!("Day 7!");

    let data: &'static str = include_str!("data.txt");
    let input: Vec<&str> = data.lines().collect();

    match part {
        1 => part1::run(input),
        2 => println!("Part 2!"),
        _ => println!("Invalid command line argument"),
    }
}
