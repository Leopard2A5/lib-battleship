use super::cell::Cell;

/// Represents a ship in the game.
#[derive(Debug, PartialEq)]
pub struct Ship<'a> {
    length: usize,
    cells: Vec<&'a Cell<'a>>,
}

impl<'a> Ship<'a> {
    /// Create a new ship.
    ///
    /// Arguments:
    ///
    /// * `length` the length of the ship
    pub fn new(length: usize) -> Ship<'a> {
        Ship {
            length: length,
            cells: Vec::new(),
        }
    }

    /// Get this ship's length.
    pub fn length(&self) -> usize {
        self.length
    }
}
