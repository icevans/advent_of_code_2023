use day_06::parse_race_part_2;

fn main() {
    let input = include_str!("../input.txt");
    let race = parse_race_part_2(input);

    println!("{}", race.winning_button_hold_times.len());
}
