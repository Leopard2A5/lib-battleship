/// The possible orientations that a ship can have on the battlefield.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Represents a ship in the game.
#[derive(Debug, PartialEq)]
pub struct Ship {
    length: usize,
    orientation: Orientation,
}

impl Ship {
    /// Create a new ship.
    ///
    /// Arguments:
    ///
    /// * `length` the length of the ship
    pub fn new(length: usize,
               orientation: Orientation) -> Ship {
        Ship {
            length: length,
            orientation: orientation
        }
    }

    /// Get this ship's length.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get this ship's orientation.
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}
