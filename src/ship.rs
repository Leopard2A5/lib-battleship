use super::cell::Cell;

/// Represents a ship in the game.
#[derive(Debug, PartialEq)]
pub struct Ship {
    length: usize
}

impl Ship {
    /// Create a new ship.
    ///
    /// Arguments:
    ///
    /// * `length` the length of the ship
    pub fn new(length: usize) -> Ship {
        Ship {
            length: length,
        }
    }

    /// Get this ship's length.
    pub fn length(&self) -> usize {
        self.length
    }
}
