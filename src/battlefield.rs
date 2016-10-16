use super::cell::Cell;
use super::ship::Ship;
use super::ship::Orientation::*;
use self::PlaceResultErr::*;
use self::ShotResultOk::*;
use std::cmp;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlaceResultErr {
    OutOfBounds,
    CellOccupied,
}

/// The possible results of placing a ship on the battlefield.
pub type PlaceResult = Result<(), PlaceResultErr>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotResultOk {
    Miss,
    Hit,
    ShipDestroyed,
    WinningShot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotResultErr {
    OutOfBounds,
    AlreadyShot,
}

/// The possible outcomes of shooting at a cell on the battlefield.
pub type ShotResult = Result<ShotResultOk, ShotResultErr>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BattlefieldCreationResultErr {
    IllegalDimensions,
}

/// The battlefield, central point of every game of battleship.
#[derive(Debug, PartialEq)]
pub struct Battlefield<'a> {
    cells: Vec<Vec<Cell<'a>>>,
    width: usize,
    height: usize,
}

impl<'a> Battlefield<'a> {
    /// Create a new Battlefield.
    pub fn new(width: usize,
               height: usize) -> Result<Battlefield<'a>, BattlefieldCreationResultErr> {
        if width < 1 || height < 2 {
            Err(BattlefieldCreationResultErr::IllegalDimensions)
        } else {
            Ok(Battlefield {
                cells: Battlefield::init_cells(width, height),
                width: width,
                height: height,
            })
        }
    }

    fn init_cells(width: usize,
                   height: usize) -> Vec<Vec<Cell<'a>>> {
        let mut ret = Vec::new();
        for _ in 0..height {
            let mut line = Vec::new();
            for _ in 0..width {
                line.push(Cell::new())
            }
            ret.push(line);
        }
        ret
    }

    /// Returns the battlefield's width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the battlefield's height.
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn shoot(&mut self,
                 x: usize,
                 y: usize) -> ShotResult {
        let mut opt_ship = None;
        let ret = self.cells.get_mut(y)
            .and_then(|row| row.get_mut(x))
            .ok_or(ShotResultErr::OutOfBounds)
            .and_then(|cell| {
                if cell.is_shot() {
                    Err(ShotResultErr::AlreadyShot)
                } else {
                    cell.shoot();
                    cell.get_ship()
                        .map_or(Ok(Miss), |ship| {
                            opt_ship = Some(ship);
                            Ok(Hit)
                        })
                }
            });

        if let Some(ship) = opt_ship {
            if self.is_ship_destroyed(ship, x, y) {
                if self.all_ships_destroyed() {
                    return Ok(WinningShot)
                } else {
                    return Ok(ShipDestroyed)
                }
            }
            return Ok(Hit)
        }

        ret
    }

    fn is_ship_destroyed(&self,
                         ship: &Ship,
                         x: usize,
                         y: usize) -> bool {
        match ship.orientation() {
            Horizontal => {
                let min = cmp::max(0, (x as i8) - (ship.length() as i8)) as usize;
                let max = cmp::min(self.width, x + ship.length());
                for i in min..max {
                    let cell = self.cells.get(y).unwrap().get(i).unwrap();
                    if let Some(cur_ship) = cell.get_ship() {
                        if ship == cur_ship && !cell.is_shot() {
                            return false
                        }
                    }
                }
                return true
            },
            Vertical => {
                let min = cmp::max(0, (y as i8) - (ship.length() as i8)) as usize;
                let max = cmp::min(self.height, x + ship.length());
                for i in min..max {
                    let cell = self.cells.get(i).unwrap().get(x).unwrap();
                    if let Some(cur_ship) = cell.get_ship() {
                        if ship == cur_ship && !cell.is_shot() {
                            return false
                        }
                    }
                }
                return true
            }
        }
    }

    fn all_ships_destroyed(&self) -> bool {
        for row in self.cells.iter() {
            for cell in row.iter() {
                if let Some(_) = cell.get_ship() {
                    if !cell.is_shot() {
                        return false
                    }
                }
            }
        }

        true
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

        if max_x < self.width && max_y < self.height {
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
    use super::Battlefield;
    use super::PlaceResultErr::*;
    use super::super::ship::Ship;
    use super::super::ship::Orientation::*;
    use super::ShotResultOk::*;
    use super::ShotResultErr;
    use super::BattlefieldCreationResultErr::IllegalDimensions;

    #[test]
    fn assert_battlefield_constructor_checks_dimensions() {
        assert_eq!(Err(IllegalDimensions), Battlefield::new(0, 0));
        assert_eq!(Err(IllegalDimensions), Battlefield::new(0, 5));
        assert_eq!(Err(IllegalDimensions), Battlefield::new(5, 0));
    }

    #[test]
    fn assert_battlefield_returns_dimensions() {
        let bf = Battlefield::new(1, 2).expect("Must work");
        assert_eq!(1, bf.width());
        assert_eq!(2, bf.height());

        let bf = Battlefield::new(4, 7).expect("Must work");
        assert_eq!(4, bf.width());
        assert_eq!(7, bf.height());
    }

    #[test]
    fn assert_new_battlefield_has_correct_size() {
        let bf = Battlefield::new(10, 10).expect("Must work");
        assert_eq!(bf.cells.len(), 10);
        for vec in bf.cells.iter() {
            assert_eq!(vec.len(), 10);
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
        let mut bf = Battlefield::new(10, 10).expect("Must work");

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
        let mut bf = Battlefield::new(10, 10).expect("Must work");

        assert_eq!(Ok(()), bf.place_ship(&ship1, 0, 0));
        assert_eq!(Err(CellOccupied), bf.place_ship(&ship2, 2, 0));
        assert_eq!(Err(CellOccupied), bf.place_ship(&ship3, 2, 0));
    }

    #[test]
    fn assert_shooting_out_of_bounds_is_an_error() {
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        assert_eq!(Err(ShotResultErr::OutOfBounds), bf.shoot(3, 0));
        assert_eq!(Err(ShotResultErr::OutOfBounds), bf.shoot(0, 3));
    }

    #[test]
    fn assert_shooting_at_empty_cells_is_a_miss() {
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        assert_eq!(Ok(Miss), bf.shoot(0, 0));
        assert_eq!(Ok(Miss), bf.shoot(1, 2));
    }

    #[test]
    fn assert_shooting_at_empty_cells_twice_is_an_error() {
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        bf.shoot(0, 0).expect("Must work");
        assert_eq!(Err(ShotResultErr::AlreadyShot), bf.shoot(0, 0));
    }

    #[test]
    fn assert_shooting_at_filled_cells_is_a_hit() {
        let ship = Ship::new(2, Horizontal);
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        bf.place_ship(&ship, 0, 0).expect("Must work");

        assert_eq!(Ok(Hit), bf.shoot(0, 0));
    }

    #[test]
    fn assert_shooting_at_filled_cells_twice_is_an_error() {
        let ship = Ship::new(2, Horizontal);
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        bf.place_ship(&ship, 0, 0).expect("Must work");

        bf.shoot(0, 0).expect("must work");
        assert_eq!(Err(ShotResultErr::AlreadyShot), bf.shoot(0, 0));
    }

    #[test]
    fn assert_destroying_a_ship_returns_ship_destroyed() {
        let ship1 = Ship::new(2, Horizontal);
        let ship2 = Ship::new(1, Horizontal);
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        bf.place_ship(&ship1, 0, 0).expect("Must work");
        // we need a second, intact ship, or we will get WinningShot instead of ShipDestroyed
        bf.place_ship(&ship2, 0, 1).expect("Must work");

        bf.shoot(0, 0).expect("must work");
        assert_eq!(Ok(ShipDestroyed), bf.shoot(1, 0));
    }

    #[test]
    fn assert_destroying_last_ship_returns_winning_shot() {
        let ship = Ship::new(1, Horizontal);
        let mut bf = Battlefield::new(3, 3).expect("Must work");
        bf.place_ship(&ship, 0, 0).expect("Must work");

        assert_eq!(Ok(WinningShot), bf.shoot(0, 0));
    }
}
