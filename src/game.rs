use ship_type::ShipType;
use battlefield::Battlefield;
use player::Player::{self, P1};
use errors::ShootError;
use errors::ShootError::*;
use super::Dimension;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShootOk {
    Hit,
    Miss,
    Destroyed,
}

#[derive(PartialEq, Debug)]
pub struct Game {
    ship_types: Vec<ShipType>,
    battlefields: Vec<Battlefield>,
    current_player: Player,
}

impl Game {
    pub fn new(
        ship_types: Vec<ShipType>,
        battlefields: Vec<Battlefield>,
    ) -> Self {
        Game {
            ship_types: ship_types,
            battlefields: battlefields,
            current_player: P1,
        }
    }

    pub fn width(&self) -> Dimension {
        self.battlefields.first().unwrap().width()
    }

    pub fn height(&self) -> Dimension {
        self.battlefields.first().unwrap().height()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn shoot(
        &mut self,
        target_player: Player,
        x: Dimension,
        y: Dimension,
    ) -> Result<ShootOk, ShootError> {
        if self.current_player == target_player {
            return Err(NotThisPlayersTurn);
        }
        let bf_num = if target_player == P1 {0} else {1};
        let cell = self.battlefields.get_mut(bf_num).unwrap()
            .get_mut_cell(x, y)
            .ok_or(OutOfBounds)?;
        cell.shoot();

        self.current_player = self.current_player.next();

        Ok(cell.ship_type_id()
            .map_or(ShootOk::Miss, |_| ShootOk::Hit))
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
    use game::ShootOk::*;

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
    fn should_tell_whos_turn_it_is() {
        let mut game = build_test_game();

        assert_eq!(P1, game.current_player());
        game.shoot(P2, 0, 0).unwrap();
        assert_eq!(P2, game.current_player());
    }

    #[test]
    fn should_respect_order_of_play() {
        let mut game = build_test_game();

        assert_eq!(Err(NotThisPlayersTurn), game.shoot(P1, 0, 0));
        game.shoot(P2, 0, 0).unwrap();
        assert_eq!(Err(NotThisPlayersTurn), game.shoot(P2, 0, 0));
        game.shoot(P1, 0, 0).unwrap();
    }

    #[test]
    fn shooting_out_of_bounds_is_an_error() {
        let mut game = build_test_game();

        assert_eq!(Err(OutOfBounds), game.shoot(P2, 3, 0));
        assert_eq!(Err(OutOfBounds), game.shoot(P2, 0, 3));
        game.shoot(P2, 0, 0).unwrap();
    }

    #[test]
    fn shooting_at_empty_cells_is_a_miss() {
        let mut game = build_test_game();

        assert_eq!(Ok(Miss), game.shoot(P2, 1, 1));
        assert_eq!(Ok(Miss), game.shoot(P1, 1, 1));
    }

    #[test]
    fn shooting_at_filled_cells_is_a_hit() {
        let mut game = build_test_game();

        assert_eq!(Ok(Hit), game.shoot(P2, 0, 0));
    }

    #[test]
    fn destroying_a_ship_returns_ship_destroyed() {
        let mut game = build_test_game();

        game.shoot(P2, 0, 0).unwrap();
        game.shoot(P1, 0, 0).unwrap();
        assert_eq!(Ok(Destroyed), game.shoot(P2, 1, 0));
    }

    fn build_test_game() -> Game {
        let mut pregame = PreGame::new(3, 3).unwrap();
        let corvette_id = pregame.add_ship_type("Corvette", 2).unwrap();
        let submarine_id = pregame.add_ship_type("Submarine", 1).unwrap();
        pregame.place_ship(P1, corvette_id, 0, 0 , Horizontal).unwrap();
        pregame.place_ship(P2, corvette_id, 0, 0 , Horizontal).unwrap();
        pregame.place_ship(P1, submarine_id, 0, 1, Horizontal).unwrap();
        pregame.place_ship(P2, submarine_id, 0, 1, Horizontal).unwrap();

        pregame.start().unwrap()
    }
}
