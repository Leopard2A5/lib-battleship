use super::cell::Cell;
use super::ship::Ship;

const X: usize = 10;
const Y: usize = 10;

/// The battlefield, central point of every game of battleship.
pub struct Battlefield<'a> {
    cells: Vec<Vec<Cell<'a>>>,
}

/// The possible results of placing a ship on the battlefield. No data is being passed
/// here, it's just about success or failure.
pub type PlaceResult = Result<(), ()>;

/// The possible orientations that a ship can have on the battlefield.
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl<'a> Battlefield<'a> {
    /// Create a new Battlefield.
    pub fn new() -> Battlefield<'a> {
        Battlefield { cells: Battlefield::init_cells() }
    }

    fn init_cells() -> Vec<Vec<Cell<'a>>> {
        let mut ret = Vec::new();
        for y in 0..Y {
            let mut line = Vec::new();
            for x in 0..X {
                line.push(Cell::new())
            }
            ret.push(line);
        }
        ret
    }

    /// Place a ship on the battlefield. Results in an Ok if the ship could be
    /// placed at the given coordinates with the given orientation. Otherwirse
    /// returns an Err.
    ///
    /// Arguments:
    ///
    /// * `ship` The ship
    /// * `x` The x coordinate
    /// * `y` The y coordinate
    /// * `orientation` The orientation
    pub fn place_ship(&mut self,
                      ship: &mut Ship,
                      x: usize,
                      y: usize,
                      orientation: Orientation)
                      -> PlaceResult {
        if !self.check_placement_in_bounds(ship, x, y, orientation) {
            Err(())
        } else {
            Ok(())
        }
    }

    fn check_placement_in_bounds(&self,
                                 ship: &Ship,
                                 x: usize,
                                 y: usize,
                                 orientation: Orientation)
                                 -> bool {
        let max_x = match orientation {
            Orientation::Horizontal => x + ship.length() - 1,
            Orientation::Vertical => x,
        };
        let max_y = match orientation {
            Orientation::Horizontal => y,
            Orientation::Vertical => y + ship.length() - 1,
        };

        return max_x < X && max_y < Y;
    }
}

#[cfg(test)]
mod tests {
    use super::{Battlefield, Orientation, X, Y};
    use super::super::ship::Ship;

    #[test]
    fn assert_new_battlefield_has_correct_size() {
        let bf = Battlefield::new();
        assert_eq!(bf.cells.len(), Y);
        for vec in bf.cells.iter() {
            assert_eq!(vec.len(), X);
        }
    }

    #[test]
    fn assert_ship_placement_only_in_bounds() {
        let mut ship = Ship::new(3);
        let mut bf = Battlefield::new();

        assert_eq!(Ok(()), bf.place_ship(&mut ship, 0, 0, Orientation::Horizontal));
        assert_eq!(Ok(()), bf.place_ship(&mut ship, 5, 5, Orientation::Vertical));

        assert_eq!(Ok(()), bf.place_ship(&mut ship, 7, 0, Orientation::Horizontal));
        assert_eq!(Err(()), bf.place_ship(&mut ship, 8, 0, Orientation::Horizontal));
        assert_eq!(Ok(()), bf.place_ship(&mut ship, 8, 0, Orientation::Vertical));

        assert_eq!(Ok(()), bf.place_ship(&mut ship, 0, 7, Orientation::Vertical));
        assert_eq!(Err(()), bf.place_ship(&mut ship, 0, 8, Orientation::Vertical));
        assert_eq!(Ok(()), bf.place_ship(&mut ship, 0, 8, Orientation::Horizontal));
    }
}
