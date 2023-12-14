use day_09::{difference_sequences, extrapolate_backwards};

fn main() {
    let input = include_str!("../input.txt");

    let extrapolations_sum: i32 = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|sequence| extrapolate_backwards(difference_sequences(sequence)))
        .sum();

    println!("{:?}", extrapolations_sum)
}