use day_03::number::Number;
use day_03::schematic::Schematic;

fn main() {
    let input = include_str!("../input.txt");
    let schematic = Schematic::from(input);

    let stars: Vec<_> = schematic
        .symbols
        .iter()
        .filter(|symbol| symbol.value == '*')
        .collect();

    let mut gear_ratios = vec![];
    for star in stars {
        let adjacent_part_numbers: Vec<&Number> = schematic
            .numbers
            .iter()
            .filter(|number| number.is_adjacent(star))
            .collect();

        if adjacent_part_numbers.len() == 2 {
            gear_ratios.push(adjacent_part_numbers[0].value * adjacent_part_numbers[1].value)
        }
    }

    println!("{:?}", gear_ratios.iter().sum::<usize>());
}
