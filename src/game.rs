use errors::GameError::{self, IllegalDimensions};

type Dimension = usize;

#[derive(PartialEq, Debug)]
pub struct Game {

}

impl Game {
    pub fn new(
        width: Dimension,
        height: Dimension,
    ) -> Result<Game, GameError> {
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
    fn assert_game_constructor_checks_dimensions() {
        assert_eq!(Err(IllegalDimensions), Game::new(0, 0));
        assert_eq!(Err(IllegalDimensions), Game::new(0, 5));
        assert_eq!(Err(IllegalDimensions), Game::new(5, 0));
    }
}
