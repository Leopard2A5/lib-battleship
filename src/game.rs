use errors::GameError;
use errors::PlaceError::{self, AlreadyPlaced, UnknownShipTypeId, OutOfBounds};
use ship_type::ShipType;
use orientation::Orientation;
use player::Player;
use battlefield::Battlefield;
use std::collections::HashSet;

pub type Dimension = usize;
pub type ShipTypeId = usize;

#[derive(PartialEq, Debug)]
pub struct Game {
    width: Dimension,
    height: Dimension,
    ship_types: Vec<ShipType>,
    placed_ships: HashSet<(Player, ShipTypeId)>,
    battlefields: Vec<Battlefield>,
}

impl Game {
    pub fn new(
        width: Dimension,
        height: Dimension,
    ) -> Result<Self, GameError> {
        let bf1 = Battlefield::new(width, height)?;
        let bf2 = bf1.clone();
        Ok(Game {
            width: width,
            height: height,
            ship_types: Vec::new(),
            placed_ships: HashSet::new(),
            battlefields: vec!(bf1, bf2),
        })
    }

    pub fn width(&self) -> Dimension {
        self.width
    }

    pub fn height(&self) -> Dimension {
        self.height
    }

    pub fn add_ship_type(
        &mut self,
        name: &'static str,
        length: Dimension,
    ) -> ShipTypeId {
        let typ = ShipType::new(name, length);
        self.ship_types.push(typ);

        return self.ship_types.len() - 1;
    }

    pub fn ship_types(&self) -> Vec<ShipType> {
        self.ship_types.clone()
    }

    pub fn place_ship(
        &mut self,
        player: Player,
        ship_type_id: ShipTypeId,
        x: Dimension,
        y: Dimension,
        orientation: Orientation,
    ) -> Result<(), PlaceError> {
        let ship_type = self.ship_types
            .get(ship_type_id)
            .ok_or(UnknownShipTypeId)?;
        self.assert_ship_not_yet_placed(player, ship_type_id)?;
        self.assert_ship_placement_in_bounds(&ship_type, x, y, orientation)?;

        self.placed_ships.insert((player, ship_type_id));
        Ok(())
    }

    fn assert_ship_not_yet_placed(
        &self,
        player: Player,
        ship_type_id: ShipTypeId,
    ) -> Result<(), PlaceError> {
        let entry = (player, ship_type_id);
        if self.placed_ships.contains(&entry) {
            Err(AlreadyPlaced)
        } else {
            Ok(())
        }
    }

    fn assert_ship_placement_in_bounds(
        &self,
        ship_type: &ShipType,
        x: Dimension,
        y: Dimension,
        orientation: Orientation,
    ) -> Result<(), PlaceError> {
        let max_x = match orientation {
            Orientation::Horizontal => x + ship_type.length() - 1,
            Orientation::Vertical => x,
        };
        let max_y = match orientation {
            Orientation::Horizontal => y,
            Orientation::Vertical => y + ship_type.length() - 1,
        };

        if max_x < self.width && max_y < self.height {
            Ok(())
        } else {
            Err(OutOfBounds)
        }
    }
}

#[cfg(test)]
mod test {
    use errors::GameError::IllegalDimensions;
    use errors::PlaceError::*;
    use player::Player::*;
    use orientation::Orientation::*;
    use game::Game;

    #[test]
    fn constructor_should_check_dimensions() {
        assert_eq!(Err(IllegalDimensions), Game::new(0, 0));
        assert_eq!(Err(IllegalDimensions), Game::new(0, 5));
        assert_eq!(Err(IllegalDimensions), Game::new(5, 0));
    }

    #[test]
    fn should_return_dimensions() {
        let game = Game::new(2, 3).unwrap();
        assert_eq!(2, game.width());
        assert_eq!(3, game.height());
    }

    #[test]
    fn should_allow_adding_ship_types() {
        let mut game = Game::new(3, 3).unwrap();
        assert_eq!(0, game.ship_types().len());

        let ship_type_id = game.add_ship_type("Corvette", 2);
        assert_eq!(0, ship_type_id);
        assert_eq!(1, game.ship_types().len());

        let types = game.ship_types();
        let st = types.first().unwrap();
        assert_eq!("Corvette", st.name());
        assert_eq!(2, st.length());
    }

    #[test]
    fn should_allow_placing_ships() {
        let mut game = Game::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2);

        assert_eq!(Ok(()), game.place_ship(P1, corvette_id, 0, 0, Horizontal));
        assert_eq!(Ok(()), game.place_ship(P2, corvette_id, 0, 0, Vertical));
    }

    #[test]
    fn should_disallow_placing_ships_twice() {
        let mut game = Game::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2);

        assert_eq!(Ok(()), game.place_ship(P1, corvette_id, 0, 0, Horizontal));
        assert_eq!(Err(AlreadyPlaced), game.place_ship(P1, corvette_id, 0, 1, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_out_of_bounds() {
        let mut game = Game::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2);

        assert_eq!(Err(OutOfBounds), game.place_ship(P1, corvette_id, 2, 0, Horizontal));
        assert_eq!(Err(OutOfBounds), game.place_ship(P1, corvette_id, 0, 2, Vertical));
        assert_eq!(Ok(()), game.place_ship(P1, corvette_id, 1, 0, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_on_top_of_each_other() {
        let mut game = Game::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2);
        let frigate_id = game.add_ship_type("Frigate", 2);

        assert_eq!(Ok(()), game.place_ship(P2, corvette_id, 0, 0, Horizontal));
        assert_eq!(Err(CellOccupied), game.place_ship(P2, frigate_id, 1, 0, Vertical));
    }
}
