use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let node_regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    let mut lines = input.lines();
    let mut directions = lines.next().unwrap().chars().cycle();
    lines.next(); // skip blank line

    let mut network = HashMap::new();
    for line in lines {
        let (_, [node, left, right]) = node_regex.captures(line).unwrap().extract::<3>();
        network.insert(node, (left, right));
    }

    let mut count = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        count += 1;
        let direction = directions.next().unwrap();
        current_node = match direction {
            'L' => network.get(current_node).unwrap().0,
            'R' => network.get(current_node).unwrap().1,
            _ => panic!("Ack! Got a non-direction: {direction}"),
        };
    }

    println!("{count}");
}
