use super::cell::Cell;
use super::ship::Ship;
use super::ship::Orientation::*;
use self::PlaceResultErr::*;

const X: usize = 10;
const Y: usize = 10;

/// The battlefield, central point of every game of battleship.
pub struct Battlefield<'a> {
    cells: Vec<Vec<Cell<'a>>>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlaceResultErr {
    OutOfBounds,
    CellOccupied
}

/// The possible results of placing a ship on the battlefield. No data is being passed
/// here, it's just about success or failure.
pub type PlaceResult = Result<(), PlaceResultErr>;

impl<'a> Battlefield<'a> {
    /// Create a new Battlefield.
    pub fn new() -> Battlefield<'a> {
        Battlefield { cells: Battlefield::init_cells() }
    }

    fn init_cells() -> Vec<Vec<Cell<'a>>> {
        let mut ret = Vec::new();
        for _ in 0..Y {
            let mut line = Vec::new();
            for _ in 0..X {
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
                      ship: &'a Ship,
                      x: usize,
                      y: usize)
                      -> PlaceResult {
        try!(self.check_placement_in_bounds(ship, x, y));
        try!(self.check_placement_against_placed_ships(ship, x, y));
        self.do_place_ship(ship, x, y);
        Ok(())
    }

    fn check_placement_in_bounds(&self,
                                 ship: &Ship,
                                 x: usize,
                                 y: usize)
                                 -> PlaceResult {
        let max_x = match ship.orientation() {
            Horizontal => x + ship.length() - 1,
            Vertical => x,
        };
        let max_y = match ship.orientation() {
            Horizontal => y,
            Vertical => y + ship.length() - 1,
        };

        if max_x < X && max_y < Y {
            Ok(())
        } else {
            Err(OutOfBounds)
        }
    }

    fn check_placement_against_placed_ships(&self,
                                            ship: &Ship,
                                            x: usize,
                                            y: usize)
                                            -> PlaceResult {
        match ship.orientation() {
            Horizontal => {
                for i in x..(x + ship.length()) {
                    let rowref = self.cells.get(y).unwrap();
                    let cellref: &Cell = rowref.get(i).unwrap();
                    if cellref.get_ship().is_some() {
                        return Err(CellOccupied)
                    }
                }
            },
            Vertical => {
                for i in y..(y + ship.length()) {
                    let rowref = self.cells.get(i).unwrap();
                    let cellref: &Cell = rowref.get(x).unwrap();
                    if cellref.get_ship().is_some() {
                        return Err(CellOccupied)
                    }
                }
            }
        }

        Ok(())
    }

    fn do_place_ship(&mut self,
                     ship: &'a Ship,
                     x: usize,
                     y: usize) {
        match ship.orientation() {
            Horizontal => {
                for i in x..(x + ship.length()) {
                    let rowref = self.cells.get_mut(y).unwrap();
                    let cellref: &mut Cell = rowref.get_mut(i).unwrap();
                    cellref.set_ship(ship);
                }
            },
            Vertical => {
                for i in y..(y + ship.length()) {
                    let rowref = self.cells.get_mut(i).unwrap();
                    let cellref: &mut Cell = rowref.get_mut(x).unwrap();
                    cellref.set_ship(ship);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Battlefield, X, Y};
    use super::PlaceResultErr::*;
    use super::super::ship::Ship;
    use super::super::ship::Orientation::*;

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
        let ship1 = Ship::new(3, Horizontal);
        let ship2 = Ship::new(3, Vertical);
        let ship3 = Ship::new(3, Horizontal);
        let ship4 = Ship::new(3, Horizontal);
        let ship5 = Ship::new(3, Vertical);
        let ship6 = Ship::new(3, Vertical);
        let ship7 = Ship::new(3, Vertical);
        let ship8 = Ship::new(3, Horizontal);
        let mut bf = Battlefield::new();

        assert_eq!(Ok(()), bf.place_ship(&ship1, 0, 0));
        assert_eq!(Ok(()), bf.place_ship(&ship2, 5, 5));

        assert_eq!(Ok(()), bf.place_ship(&ship3, 7, 0));
        assert_eq!(Err(OutOfBounds), bf.place_ship(&ship4, 8, 0));
        assert_eq!(Ok(()), bf.place_ship(&ship5, 8, 1));

        assert_eq!(Ok(()), bf.place_ship(&ship6, 0, 7));
        assert_eq!(Err(OutOfBounds), bf.place_ship(&ship7, 1, 8));
        assert_eq!(Ok(()), bf.place_ship(&ship8, 1, 8));
    }

    #[test]
    fn assert_ship_placement_against_set_ships() {
        let ship1 = Ship::new(3, Horizontal);
        let ship2 = Ship::new(3, Horizontal);
        let ship3 = Ship::new(3, Vertical);
        let mut bf = Battlefield::new();

        assert_eq!(Ok(()), bf.place_ship(&ship1, 0, 0));
        assert_eq!(Err(CellOccupied), bf.place_ship(&ship2, 2, 0));
        assert_eq!(Err(CellOccupied), bf.place_ship(&ship3, 2, 0));
    }
}
