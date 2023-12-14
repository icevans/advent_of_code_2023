use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Number {
    pub x_start: usize,
    pub x_end: usize,
    pub y: usize,
    pub value: usize,
}

impl Number {
    // TODO: How do I make this more generic to not demand a symbol, but instead
    //       just want something that has an x and a y?
    pub fn is_adjacent(&self, symbol: &Symbol) -> bool {
        self.y.checked_sub(1).unwrap_or(0) <= symbol.y
            && symbol.y <= self.y + 1
            && self.x_start.checked_sub(1).unwrap_or(0) <= symbol.x
            && symbol.x <= self.x_end + 1
    }
}

#[cfg(test)]
mod test {
    use super::{Number, Symbol};

    #[test]
    fn adjacent() {
        let number = Number {
            x_start: 2,
            x_end: 4,
            y: 2,
            value: 456,
        };
        let mut symbols: Vec<Symbol> = Vec::new();

        for x in 1..=5 {
            for y in 1..=3 {
                symbols.push(Symbol::new(x, y, '*'))
            }
        }

        for symbol in symbols {
            assert_eq!(true, number.is_adjacent(&symbol))
        }
    }

    #[test]
    fn is_not_adjacent() {
        let number = Number {
            x_start: 2,
            x_end: 4,
            y: 2,
            value: 456,
        };
        let mut symbols: Vec<Symbol> = Vec::new();

        for x in 0..=6 {
            for y in 0..=4 {
                if y == 1 || y == 2 || y == 3 {
                    if x == 0 || x == 6 {
                        symbols.push(Symbol::new(x, y, '*'))
                    }
                } else {
                    symbols.push(Symbol::new(x, y, '*'))
                }
            }
        }

        for symbol in symbols {
            assert_eq!(false, number.is_adjacent(&symbol))
        }
    }
}