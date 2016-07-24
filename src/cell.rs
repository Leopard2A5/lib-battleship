use super::ship::Ship;

#[derive(Debug, PartialEq)]
pub struct Cell<'a> {
    ship: Option<&'a Ship<'a>>,
    shot: bool,
}

impl<'a> Cell<'a> {
    pub fn new() -> Cell<'a> {
        Cell {
            ship: None,
            shot: false,
        }
    }

    pub fn shoot(&mut self) {
        self.shot = true;
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
