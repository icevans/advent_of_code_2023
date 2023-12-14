use num::integer;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let node_regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    let mut lines = input.lines();
    let mut directions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next(); // skip blank line

    let mut network = HashMap::new();
    let mut current_positions = vec![];
    for line in lines {
        let (_, [node, left, right]) = node_regex.captures(line).unwrap().extract::<3>();
        network.insert(node.clone(), (left, right));

        if node.ends_with("A") {
            current_positions.push(node);
        }
    }

    let mut counts = Vec::new();

    for starting_position in &current_positions {
        let directions = directions.iter().cycle();
        let mut current_position = starting_position;
        let mut count = 0;
        for direction in directions {
            current_position = match direction {
                'L' => &network.get(current_position).unwrap().0,
                'R' => &network.get(current_position).unwrap().1,
                _ => panic!("Ack! Got a non-direction: {direction}"),
            };
            count += 1;
            if current_position.ends_with("Z") {
                break;
            }
        }
        counts.push(count as u64)
    }

    let min_steps = counts
        .into_iter()
        .reduce(|current_lcm, y| integer::lcm(current_lcm, y)).unwrap();

    println!("{:?}", min_steps);
}
