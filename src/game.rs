use ship_type::ShipType;
use battlefield::Battlefield;
use player::Player;
use errors::ShootError;
use super::Dimension;

#[derive(PartialEq, Debug)]
pub struct Game {
    ship_types: Vec<ShipType>,
    battlefields: Vec<Battlefield>,
}

impl Game {
    pub fn new(
        ship_types: Vec<ShipType>,
        battlefields: Vec<Battlefield>,
    ) -> Self {
        Game {
            ship_types: ship_types,
            battlefields: battlefields,
        }
    }

    pub fn width(&self) -> Dimension {
        self.battlefields.first().unwrap().width()
    }

    pub fn height(&self) -> Dimension {
        self.battlefields.first().unwrap().height()
    }

    pub fn shoot(
        &mut self,
        target_player: Player,
        x: Dimension,
        y: Dimension,
    ) -> Result<(), ShootError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use battlefield::Battlefield;
    use errors::ShootError::*;
    use game::Game;
    use orientation::Orientation::*;
    use player::Player::*;
    use pregame::PreGame;
    use ship_type::ShipType;

    #[test]
    fn should_return_dimensions() {
        let ship_types = vec!(ShipType::new("Corvette", 2));
        let bf1 = Battlefield::new(2, 3).unwrap();
        let bf2 = bf1.clone();
        let battlefields = vec!(bf1, bf2);

        let game = Game::new(ship_types, battlefields);
        assert_eq!(2, game.width());
        assert_eq!(3, game.height());
    }

    #[test]
    fn assert_shooting_out_of_bounds_is_an_error() {
        let mut game = build_test_game();

        assert_eq!(Err(OutOfBounds), game.shoot(P2, 3, 0));
        assert_eq!(Err(OutOfBounds), game.shoot(P2, 0, 3));
        game.shoot(P2, 0, 0).unwrap();
    }

    fn build_test_game() -> Game {
        let mut pregame = PreGame::new(3, 3).unwrap();
        let corvette_id = pregame.add_ship_type("Corvette", 2).unwrap();
        pregame.place_ship(P1, corvette_id, 0, 0 , Horizontal).unwrap();
        pregame.place_ship(P2, corvette_id, 0, 0 , Horizontal).unwrap();

        pregame.start().unwrap()
    }
}
