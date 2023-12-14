use day_06::parse_races;

fn main() {
    let input = include_str!("../input.txt");

    let races = parse_races(input);
    let margin_of_error = races
        .iter()
        .map(|race| race.winning_button_hold_times.len() as u32)
        .product::<u32>();

    println!("{margin_of_error}");
}
