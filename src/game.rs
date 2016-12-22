use errors::GameError::{self, IllegalDimensions};

type Dimension = usize;

#[derive(PartialEq, Debug)]
pub struct Game {

}

impl Game {
    pub fn new(
        width: Dimension,
        height: Dimension,
    ) -> Result<Self, GameError> {
        if width < 2 || height < 2 {
            Err(IllegalDimensions)
        } else {
            Ok(Game {})
        }
    }
}

#[cfg(test)]
mod test {
    use errors::GameError::IllegalDimensions;
    use game::Game;

    #[test]
    fn game_constructor_should_check_dimensions() {
        assert_eq!(Err(IllegalDimensions), Game::new(0, 0));
        assert_eq!(Err(IllegalDimensions), Game::new(0, 5));
        assert_eq!(Err(IllegalDimensions), Game::new(5, 0));
    }

}
