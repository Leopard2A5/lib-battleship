use super::cell::Cell;

#[derive(Debug, PartialEq)]
pub struct Ship<'a> {
    length: usize,
    cells: Vec<&'a Cell<'a>>,
}

impl<'a> Ship<'a> {
    pub fn new(length: usize) -> Ship<'a> {
        Ship {
            length: length,
            cells: Vec::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}
