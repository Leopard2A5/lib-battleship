//! Implementations for a started game of battleship.

use battlefield::Battlefield;
use battlefield::ShipStatus;
use common::CellStatus;
use common::Dimensional;
use common::Player::{self, P1, P2};
use common::ShipType;
use common::ShipTypeContainer;
use results::ShootError;
use results::ShootError::*;
use results::ShootOk;
use results::ShootOk::*;
use super::Dimension;
use std::rc::Rc;

/// Struct representing a running game of battleship.
#[derive(PartialEq, Debug)]
pub struct Game {
    ship_types: Vec<Rc<ShipType>>,
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
        ship_types: Vec<Rc<ShipType>>,
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
    /// * `GameOver` if the game is already finished
    ///
    /// # Examples
    /// ```
    /// # use lib_battleship::common::Player::{P1, P2};
    /// # use lib_battleship::PreGame;
    /// # use lib_battleship::Game;
    /// # use lib_battleship::common::Orientation::Horizontal;
    /// # use lib_battleship::results::ShootOk;
    /// # let mut pregame = PreGame::new(3, 3).unwrap();
    /// # let corvette = pregame.add_ship_type("Corvette", 2).unwrap();
    /// # pregame.place_ship(P1, &corvette, 0, 0, Horizontal).unwrap();
    /// # pregame.place_ship(P2, &corvette, 0, 0, Horizontal).unwrap();
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
        if let Some(_) = self.get_winner() {
            return Err(GameOver);
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

    /// Gets the status of the cell (`x`, `y`) owned by `player`.
    /// Does not display missed shots, i.e. misses are considered `Empty` (see `get_opponent_cell`).
    /// # Parameters
    /// * `player` determines which battlefield to consider, i.e. the owner of the battlefield.
    /// * `x` the x coordinate
    /// * `y` the y coordinate
    ///
    /// # Panics
    /// Panics if the x and/or y coordinate is out of bounds.
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

    /// Gets the status of the *opponent* cell (`x`, `y`) owned by `player`.
    /// Does not display unhit ship cells, i.e. unhit ship cells are considered `Empty`.
    /// # Parameters
    /// * `player` determines which battlefield to consider, i.e. the owner of the battlefield.
    /// * `x` the x coordinate
    /// * `y` the y coordinate
    ///
    /// # Panics
    /// Panics if the x and/or y coordinate is out of bounds.
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

    /// Gets the winner of the game, if any.
    pub fn get_winner(&self) -> Option<Player> {
        if self.ship_status.get_sum_health(P1) == 0 {
            Some(P2)
        } else if self.ship_status.get_sum_health(P2) == 0 {
            Some(P1)
        } else {
            None
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

impl ShipTypeContainer for Game {
    fn ship_types(&self) -> Vec<Rc<ShipType>> {
        self.ship_types.clone()
    }
}

#[cfg(test)]
mod test {
    use battlefield::Battlefield;
    use common::CellStatus;
    use common::Dimensional;
    use common::Orientation::*;
    use common::Player::*;
    use common::ShipType;
    use common::ShipTypeContainer;
    use pregame::PreGame;
    use results::ShootError::*;
    use results::ShootOk::*;
    use super::Game;
    use std::rc::Rc;

    #[test]
    fn should_return_dimensions() {
        let ship_types = vec!(Rc::new(ShipType::new(0, "Corvette", 2)));
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

    #[test]
    fn should_return_contained_ship_types() {
        let game = build_test_game();
        let mut types = game.ship_types();

        assert_eq!(2, types.len());
        types.clear();
        assert_eq!(2, game.ship_types().len());
    }

    #[test]
    fn should_not_allow_shots_after_game_ends() {
        let mut game = build_test_game();
        game.shoot(P2, 0, 0).unwrap();
        game.shoot(P2, 1, 0).unwrap();
        game.shoot(P2, 0, 1).unwrap();

        assert_eq!(Err(GameOver), game.shoot(P2, 2, 2));
        assert_eq!(Some(P1), game.get_winner());

        let mut game = build_test_game();
        game.shoot(P2, 2, 2).unwrap();
        game.shoot(P1, 0, 0).unwrap();
        game.shoot(P1, 1, 0).unwrap();
        game.shoot(P1, 0, 1).unwrap();

        assert_eq!(Err(GameOver), game.shoot(P1, 2, 2));
        assert_eq!(Some(P2), game.get_winner());
    }

    fn build_test_game() -> Game {
        let mut pregame = PreGame::new(3, 3).unwrap();
        let corvette = pregame.add_ship_type("Corvette", 2).unwrap();
        let submarine = pregame.add_ship_type("Submarine", 1).unwrap();
        pregame.place_ship(P1, &corvette, 0, 0 , Horizontal).unwrap();
        pregame.place_ship(P2, &corvette, 0, 0 , Horizontal).unwrap();
        pregame.place_ship(P1, &submarine, 0, 1, Horizontal).unwrap();
        pregame.place_ship(P2, &submarine, 0, 1, Horizontal).unwrap();

        pregame.start().unwrap()
    }
}
