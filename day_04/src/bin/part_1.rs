use day_04::Card;

fn main() {
    let input = include_str!("../input.txt");
    let card_pile_worth: u32 = input
        .lines()
        .map(|line| Card::from(line))
        .map(|card| card.worth())
        .sum();

    println!("{card_pile_worth}");
}
