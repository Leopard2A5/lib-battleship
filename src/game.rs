use errors::GameError::{self, IllegalDimensions};
use ship_type::ShipType;

pub type Dimension = usize;

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
    ) {
        let typ = ShipType::new(name, length);
        self.ship_types.push(typ);
    }

    pub fn ship_types(&self) -> Vec<ShipType> {
        self.ship_types.clone()
    }
}

#[cfg(test)]
mod test {
    use errors::GameError::IllegalDimensions;
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

        game.add_ship_type("Corvette", 2);
        assert_eq!(1, game.ship_types().len());

        let types = game.ship_types();
        let st = types.first().unwrap();
        assert_eq!("Corvette", st.name());
        assert_eq!(2, st.length());
    }
}
