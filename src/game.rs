//! Implementations for a started game of battleship.

use battlefield::Battlefield;
use battlefield::CellStatus;
use battlefield::Dimensional;
use battlefield::ShipStatus;
use battlefield::ShipType;
use results::ShootError;
use results::ShootError::*;
use results::ShootOk;
use results::ShootOk::*;
use common::Player::{self, P1};
use super::Dimension;

/// Struct representing a running game of battleship.
#[derive(PartialEq, Debug)]
pub struct Game {
    ship_types: Vec<ShipType>,
    battlefields: Vec<Battlefield>,
    current_player: Player,
    ship_status: ShipStatus,
}

impl Game {
    /// Creates a new instance. Use of this function is discouraged. You should rather go through
    /// the `PreGame` struct.
    /// # Parameters
    /// * `ship_types` A non-emptpy vector of `ShipType`s.
    /// * `battlefields` A Vector of exactly two `Battlefield`s where player 1 owns the
    ///   battlefield at index 0 and player 2 owns the one at index 1.
    pub fn new(
        ship_types: Vec<ShipType>,
        battlefields: Vec<Battlefield>,
    ) -> Self {
        Game {
            ship_types: ship_types.clone(),
            battlefields: battlefields,
            current_player: P1,
            ship_status: ShipStatus::new(&ship_types),
        }
    }

    /// This function determines who's turn it is.
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Shoot at a player's battlefield.
    /// # Parameters
    /// * `target_player` The player to be shot at.
    /// * `x` The x coordinate.
    /// * `y` The y coordinate.
    ///
    /// # Errors
    /// * `NotThisPlayersTurn` if `target_player` is the same as what's returned by `current_player()`.
    /// * `OutOfBounds` if the given coordinates are outside the boundaries of the battlefield.
    ///
    /// # Examples
    /// ```
    /// # use lib_battleship::common::Player::{P1, P2};
    /// # use lib_battleship::pregame::PreGame;
    /// # use lib_battleship::game::Game;
    /// # use lib_battleship::common::Orientation::Horizontal;
    /// # use lib_battleship::results::ShootOk;
    /// # let mut pregame = PreGame::new(3, 3).unwrap();
    /// # let corvette_id = pregame.add_ship_type("Corvette", 2).unwrap();
    /// # pregame.place_ship(P1, corvette_id, 0, 0, Horizontal).unwrap();
    /// # pregame.place_ship(P2, corvette_id, 0, 0, Horizontal).unwrap();
    /// # let mut game = pregame.start().unwrap();
    /// #
    /// match game.shoot(P2, 0, 0).unwrap() {
    ///     ShootOk::Hit => println!("hit!"),
    ///     ShootOk::Miss => println!("miss!"),
    ///     ShootOk::Destroyed => println!("ship destroyed!"),
    ///     ShootOk::WinningShot => println!("you won!")
    /// }
    /// // note that you shouldn't just call `unwrap()` after `shoot()`, don't ignore errors.
    /// ```
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

        if let Some(ship_type_id) = cell.ship_type_id() {
            let new_health = self.ship_status.hit(target_player, ship_type_id);
            let sum_health = self.ship_status.get_sum_health(target_player);
            if sum_health <= 0 {
                Ok(WinningShot)
            } else if new_health <= 0 {
                Ok(Destroyed)
            } else {
                Ok(Hit)
            }
        } else {
            self.current_player = self.current_player.next();
            Ok(Miss)
        }
    }

    pub fn get_cell(
        &self,
        player: Player,
        x: Dimension,
        y: Dimension,
    ) -> CellStatus {
        let bf = if player == P1 {
            self.battlefields.get(0).unwrap()
        } else {
            self.battlefields.get(1).unwrap()
        };
        let cell = bf.get_cell(x, y).unwrap();
        let filled = cell.ship_type_id().is_some();
        let shot = cell.is_shot();

        if filled {
            if shot {
                CellStatus::Hit
            } else {
                CellStatus::Ship
            }
        } else {
            CellStatus::Empty
        }
    }

    pub fn get_opponent_cell(
        &self,
        player: Player,
        x: Dimension,
        y: Dimension,
    ) -> CellStatus {
        let bf = if player == P1 {
            self.battlefields.get(0).unwrap()
        } else {
            self.battlefields.get(1).unwrap()
        };
        let cell = bf.get_cell(x, y).unwrap();
        let filled = cell.ship_type_id().is_some();
        let shot = cell.is_shot();

        if shot {
            if filled {
                CellStatus::Hit
            } else {
                CellStatus::Miss
            }
        } else {
            CellStatus::Empty
        }
    }
}

impl Dimensional for Game {
    fn width(&self) -> Dimension {
        self.battlefields.first().unwrap().width()
    }

    fn height(&self) -> Dimension {
        self.battlefields.first().unwrap().height()
    }
}

#[cfg(test)]
mod test {
    use battlefield::Battlefield;
    use battlefield::CellStatus;
    use battlefield::Dimensional;
    use battlefield::ShipType;
    use results::ShootError::*;
    use results::ShootOk::*;
    use super::Game;
    use common::Orientation::*;
    use common::Player::*;
    use pregame::PreGame;

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
        game.shoot(P2, 2, 2).unwrap();
        assert_eq!(P2, game.current_player());
    }

    #[test]
    fn should_respect_order_of_play() {
        let mut game = build_test_game();

        assert_eq!(Err(NotThisPlayersTurn), game.shoot(P1, 0, 0));
        game.shoot(P2, 2, 2).unwrap();
        assert_eq!(Err(NotThisPlayersTurn), game.shoot(P2, 0, 0));
        game.shoot(P1, 0, 0).unwrap();
    }

    #[test]
    fn a_hit_lets_you_shoot_again() {
        let mut game = build_test_game();

        // single hit
        assert_eq!(P1, game.current_player());
        game.shoot(P2, 0, 0).unwrap();
        assert_eq!(P1, game.current_player());
        game.shoot(P2, 2, 2).unwrap();
        assert_eq!(P2, game.current_player());

        // destroyed ship
        game.shoot(P1, 2, 2).unwrap();
        game.shoot(P2, 1, 0).unwrap();
        assert_eq!(P1, game.current_player());
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
        assert_eq!(Ok(Destroyed), game.shoot(P2, 1, 0));
    }

    #[test]
    fn destroying_last_ship_wins_game() {
        let mut game = build_test_game();

        game.shoot(P2, 0, 0).unwrap();
        game.shoot(P2, 1, 0).unwrap();
        assert_eq!(Ok(WinningShot), game.shoot(P2, 0, 1));
    }

    #[test]
    fn can_get_cell_status_for_owner() {
        let mut game = build_test_game();

        assert_eq!(CellStatus::Empty, game.get_cell(P2, 2, 2));
        assert_eq!(CellStatus::Ship, game.get_cell(P2, 0, 0));
        game.shoot(P2, 0, 0).unwrap();
        assert_eq!(CellStatus::Hit, game.get_cell(P2, 0, 0));
    }

    #[test]
    fn owner_cell_status_doesnt_show_misses() {
        let mut game = build_test_game();

        game.shoot(P2, 2, 2).unwrap();
        assert_eq!(CellStatus::Empty, game.get_cell(P2, 2, 2));
    }

    #[test]
    fn can_get_cell_status_for_opponent() {
        let mut game = build_test_game();

        assert_eq!(CellStatus::Empty, game.get_opponent_cell(P2, 0, 0));
        game.shoot(P2, 0, 0).unwrap();
        assert_eq!(CellStatus::Hit, game.get_opponent_cell(P2, 0, 0));
        game.shoot(P2, 2, 2).unwrap();
        assert_eq!(CellStatus::Miss, game.get_opponent_cell(P2, 2, 2));
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
