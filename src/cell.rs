use super::ship::Ship;

/// Represents a cell of the battlefield. A cell is made up of an optional ship reference and
/// a bool signifying if this cell has been shot at.
#[derive(Debug, PartialEq)]
pub struct Cell<'a> {
    ship: Option<&'a Ship<'a>>,
    shot: bool,
}

impl<'a> Cell<'a> {
    /// Create a new, empty cell.
    pub fn new() -> Cell<'a> {
        Cell {
            ship: None,
            shot: false,
        }
    }

    /// Sets this cell's shot status to true.
    pub fn shoot(&mut self) {
        self.shot = true;
    }

    pub fn ship(&self) -> Option<&'a Ship<'a>> {
        self.ship
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn assert_new_cells_are_empty_and_not_shot() {
        let cell = Cell::new();
        assert_eq!(None, cell.ship);
        assert_eq!(false, cell.shot);
    }

    #[test]
    fn assert_shoot_sets_shot() {
        let mut cell = Cell::new();
        cell.shoot();
        assert!(cell.shot);
    }
}
