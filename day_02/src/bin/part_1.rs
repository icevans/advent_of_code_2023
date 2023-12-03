use day_02::cube_set::CubeSet;
use day_02::game::Game;

fn main() {
    let input = include_str!("../input.txt");

    let starting_cube_set = CubeSet {
        red: 12,
        blue: 14,
        green: 13,
    };

    let result = input
        .lines()
        .map(|game_description| Game::from(game_description))
        .filter(|game| game.is_possible_for_cube_set(&starting_cube_set))
        .fold(0, |sum, game| sum + game.id);

    println!("{result}");
}
