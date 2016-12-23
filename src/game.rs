use errors::GameError::{self, IllegalDimensions};
use errors::PlaceError;
use ship_type::ShipType;
use orientation::Orientation;
use player::Player;

pub type Dimension = usize;
pub type ShipTypeId = usize;

#[derive(PartialEq, Debug)]
pub struct Game {
    width: Dimension,
    height: Dimension,
    ship_types: Vec<ShipType>,
}

impl Game {
    pub fn new(
        width: Dimension,
        height: Dimension,
    ) -> Result<Self, GameError> {
        if width < 2 || height < 2 {
            Err(IllegalDimensions)
        } else {
            Ok(Game {
                width: width,
                height: height,
                ship_types: Vec::new(),
            })
        }
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
        shipTypeId: ShipTypeId,
        x: Dimension,
        y: Dimension,
        orientation: Orientation,
    ) -> Result<(), PlaceError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use errors::GameError::IllegalDimensions;
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
        let corvetteId = game.add_ship_type("Corvette", 2);

        assert_eq!(Ok(()), game.place_ship(P1, corvetteId, 0, 0, Horizontal));
        assert_eq!(Ok(()), game.place_ship(P2, corvetteId, 0, 0, Vertical));
    }
}
