pub mod symbol;
pub mod number;

pub mod schematic {
    use regex::Regex;
    use crate::number::Number;
    use crate::symbol::Symbol;

    #[derive(Debug)]
    pub struct Schematic {
        pub numbers: Vec<Number>,
        pub symbols: Vec<Symbol>,
    }

    impl Schematic {
        pub fn from(input: &str) -> Schematic {
            let r = Regex::new(Symbol::SYMBOL_REGEX).unwrap();
            let mut schematic = Schematic {
                numbers: vec![],
                symbols: vec![],
            };

            for (y, line) in input.lines().enumerate() {
                let characters: Vec<char> = line.chars().collect();
                let mut x = 0;

                while x < characters.len() {
                    if r.is_match(&characters[x].to_string()) {
                        schematic.symbols.push(Symbol::new(x, y, characters[x]))
                    } else if characters[x].is_numeric() {
                        let mut number = Number {
                            x_start: x,
                            x_end: 0,
                            y,
                            value: 0,
                        };

                        let mut number_string = characters[x].to_string();
                        while x + 1 < characters.len() && characters[x + 1].is_numeric() {
                            x += 1;
                            number_string.push_str(&characters[x].to_string())
                        }

                        number.x_end = x;
                        number.value = number_string.parse().unwrap();
                        schematic.numbers.push(number);
                    }

                    x += 1;
                }
            }

            schematic
        }
    }
}
