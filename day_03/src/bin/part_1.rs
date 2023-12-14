use day_03::number::Number;
use day_03::schematic::Schematic;

fn main() {
    let input = include_str!("../input.txt");
    let schematic = Schematic::from(input);

    let mut part_numbers: Vec<Number> = vec![];
    for number in schematic.numbers {
        if schematic
            .symbols
            .iter()
            .any(|symbol| number.is_adjacent(&symbol))
        {
            part_numbers.push(number)
        }
    }

    let sum: usize = part_numbers
        .iter()
        .map(|part_number| part_number.value)
        .sum();

    println!("{sum}");
}
