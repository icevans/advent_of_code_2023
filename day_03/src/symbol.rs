use crate::number::Number;

#[derive(Debug)]
pub struct Symbol {
    pub x: usize,
    pub y: usize,
    pub value: char,
}

impl Symbol {
    pub const SYMBOL_REGEX: &str = r"[^a-zA-Z0-9. ]";

    pub fn new(x: usize, y: usize, value: char) -> Symbol {
        Symbol { x, y, value }
    }
}