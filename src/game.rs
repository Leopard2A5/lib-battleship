use errors::GameError::{self, IllegalDimensions};

type Dimension = usize;

#[derive(PartialEq, Debug)]
pub struct Game {
    width: Dimension,
    height: Dimension,
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
            })
        }
    }

    pub fn width(&self) -> Dimension {
        self.width
    }

    pub fn height(&self) -> Dimension {
        self.height
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

    #[test]
    fn game_should_return_dimensions() {
        let game = Game::new(2, 3).unwrap();
        assert_eq!(2, game.width());
        assert_eq!(3, game.height());
    }
}
