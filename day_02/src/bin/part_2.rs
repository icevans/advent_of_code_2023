use day_02::game::Game;

fn main() {
    let input = include_str!("../input.txt");

    let result: usize = input
        .lines()
        .map(|game_description| Game::from(game_description))
        .map(|game| game.minimum_starting_cube_set())
        .map(|cube_set| cube_set.power())
        .sum();

    println!("{result}")
}
