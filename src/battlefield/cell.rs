use ::ShipTypeId;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    ship_type_id: Option<ShipTypeId>,
    shot: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            ship_type_id: None,
            shot: false,
        }
    }

    pub fn shoot(&mut self) {
        self.shot = true;
    }

    pub fn is_shot(&self) -> bool {
        self.shot
    }

    pub fn ship_type_id(&self) -> Option<ShipTypeId> {
        self.ship_type_id
    }

    pub fn set_ship_type_id(
        &mut self,
        ship_type_id: ShipTypeId
    ) {
        self.ship_type_id = Some(ship_type_id)
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn assert_new_cells_are_empty_and_not_shot() {
        let cell = Cell::new();
        assert_eq!(None, cell.ship_type_id);
        assert_eq!(false, cell.shot);
    }

    #[test]
    fn assert_ship_type_id_works() {
        let mut cell = Cell {
            ship_type_id: None,
            shot: false,
        };
        assert_eq!(None, cell.ship_type_id());

        cell.set_ship_type_id(7);
        assert_eq!(Some(7), cell.ship_type_id());
    }

    #[test]
    fn assert_shooting_works() {
        let mut cell = Cell::new();
        assert!(!cell.is_shot());

        cell.shoot();
        assert!(cell.is_shot());
    }

    #[test]
    fn assert_shoot_sets_shot() {
        let mut cell = Cell::new();
        cell.shoot();
        assert!(cell.shot);
    }
}
