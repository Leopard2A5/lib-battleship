//! Everything you need to set up a game of battleship.

use battlefield::Battlefield;
use common::CellStatus;
use common::Dimensional;
use common::Orientation;
use common::Player;
use common::Player::*;
use common::ShipType;
use common::ShipTypeContainer;
use game::Game;
use results::GameError;
use results::GameStartError;
use results::GameStartError::*;
use results::PlaceError;
use results::PlaceError::*;
use results::ShipTypeError;
use results::ShipTypeError::*;
use std::cmp::max;
use std::collections::HashSet;
use std::sync::Arc;
use super::Dimension;
use super::ShipTypeId;

/// Builder type for a game of battleship.
#[derive(PartialEq, Debug)]
pub struct PreGame {
    width: Dimension,
    height: Dimension,
    ship_types: Vec<Arc<ShipType>>,
    placed_ships: HashSet<(Player, ShipTypeId)>,
    battlefields: Vec<Battlefield>,
}

/// Builder style struct for battleship.
impl PreGame {
    /// Creates a new instance.
    /// # Parameters
    /// * `width` The number of columns
    /// * `height` The number of lines
    ///
    /// # Errors
    /// * `IllegalDimensions` will be returned if `width` or `height` are less than 2.
    pub fn new(
        width: Dimension,
        height: Dimension,
    ) -> Result<PreGame, GameError> {
        let bf1 = Battlefield::new(width, height)?;
        let bf2 = bf1.clone();
        Ok(PreGame {
            width: width,
            height: height,
            ship_types: Vec::new(),
            placed_ships: HashSet::new(),
            battlefields: vec!(bf1, bf2),
        })
    }

    /// Consume this `PreGame` and provide a `Game` from it.
    /// Requires that both players have placed all their ships.
    /// # Errors
    /// * `NoShipsPlaced` if no player has placed any ships yet
    /// * `NotAllShipsPlaced` if not all ships have been placed yet
    pub fn start(self) -> Result<Game, (Self, GameStartError)> {
        if self.placed_ships.len() == 0 {
            Err((self, NoShipsPlaced))
        } else if self.placed_ships.len() != (2 * self.ship_types.len()) {
            Err((self, NotAllShipsPlaced))
        } else {
            Ok(Game::new(self.ship_types, self.battlefields))
        }
    }

    /// Add a ship type to the game. Returns a unique id for the new ship type.
    /// # Parameters
    /// * `name` The name of the ship type.
    /// * `length` The length of the ship type.
    ///
    /// # Errors
    /// * `IllegalShipLength` If the ship type's length is smaller than 1.
    /// * `ShipTooLongForBattlefield` If the ship length is longer than the width or height of the battlefield.
    pub fn add_ship_type(
        &mut self,
        name: &'static str,
        length: Dimension,
    ) -> Result<Arc<ShipType>, ShipTypeError> {
        if length == 0 {
            Err(IllegalShipLength)
        } else if length > max(self.width(), self.height()) {
            Err(ShipTooLongForBattlefield)
        } else {
            let typ = ShipType::new(self.ship_types.len(), name, length);
            let rc = Arc::new(typ);
            self.ship_types.push(rc.clone());
            Ok(rc)
        }
    }

    /// Place a ship of a previously added ship type on the battlefield.
    /// # Parameters
    /// * `player` The player who owns the ship
    /// * `ship_type` Ref to the ship type of the ship to be placed.
    /// * `x` The x coordinate of the ship
    /// * `y` The y coordinate of the ship
    /// * `orientation` The orientation of the ship
    ///
    /// # Errors
    /// * `AlreadyPlaced` In case the player has already placed a ship of that ship type.
    /// * `OutOfBounds` If the ship would exceed any boundary of the battlefield.
    /// * `UnknownShipType` If the ship type id is invalid.
    /// * `CellOccupied` If the ship would occupy an already occupied coordinate.
    ///
    /// # Examples
    /// Player 1 places a corvette of length 2 on (0, 0) and (1, 0)
    ///
    /// ```
    /// # use lib_battleship::common::Orientation::Horizontal;
    /// # use lib_battleship::common::Player::P1;
    /// # use lib_battleship::PreGame;
    /// #
    /// let mut pregame = PreGame::new(3, 3).unwrap();
    /// let corvette = pregame.add_ship_type("Corvette", 2).unwrap();
    /// let result = pregame.place_ship(P1, &corvette, 0, 0, Horizontal);
    /// // check result here
    /// ```
    pub fn place_ship(
        &mut self,
        player: Player,
        ship_type: &Arc<ShipType>,
        x: Dimension,
        y: Dimension,
        orientation: Orientation,
    ) -> Result<(), PlaceError> {
        self.assert_ship_type_known(&ship_type)?;
        self.assert_ship_not_yet_placed(player, ship_type.id())?;
        self.assert_ship_placement_in_bounds(&ship_type, x, y, orientation)?;
        let affected_cell_coords = self.get_affected_cell_coords(&ship_type, x, y, orientation);
        self.assert_cells_free(player, &affected_cell_coords)?;

        self.do_place_ship(player, ship_type.id(), &affected_cell_coords);
        self.placed_ships.insert((player, ship_type.id()));
        Ok(())
    }

    fn assert_ship_type_known(
        &self,
        ship_type: &Arc<ShipType>,
    ) -> Result<(), PlaceError> {
        self.ship_types.iter()
            .find(|x| *x == ship_type )
            .ok_or(UnknownShipType)?;

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

    fn get_affected_cell_coords(
        &self,
        ship_type: &ShipType,
        x: Dimension,
        y: Dimension,
        orientation: Orientation,
    ) -> Vec<(Dimension, Dimension)> {
        match orientation {
            Orientation::Horizontal => {
                (0..ship_type.length())
                    .map(|n| x + n)
                    .map(|n| (n, y))
                    .collect()
            },
            Orientation::Vertical => {
                (0..ship_type.length())
                    .map(|n| y + n)
                    .map(|n| (x, n))
                    .collect()
            }
        }
    }

    fn assert_cells_free(
        &self,
        player: Player,
        cell_coords: &Vec<(Dimension, Dimension)>,
    ) -> Result<(), PlaceError> {
        let bf = if player == P1 {
            self.battlefields.get(0).unwrap()
        } else {
            self.battlefields.get(1).unwrap()
        };

        for coords in cell_coords {
            let (x, y) = *coords;
            let cell = bf.get_cell(x, y).unwrap();
            if cell.ship_type_id().is_some() {
                return Err(CellOccupied);
            }
        }

        Ok(())
    }

    fn do_place_ship(
        &mut self,
        player: Player,
        ship_type_id: ShipTypeId,
        affected_cell_coords: &Vec<(Dimension, Dimension)>
    ) {
        let bf = if player == P1 {
            self.battlefields.get_mut(0).unwrap()
        } else {
            self.battlefields.get_mut(1).unwrap()
        };

        for coords in affected_cell_coords {
            let (x, y) = *coords;
            let cell = bf.get_mut_cell(x, y).unwrap();
            cell.set_ship_type_id(ship_type_id);
        }
    }

    /// Gets the status of the cell (`x`, `y`) owned by `player`.
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

        if let Some(_) = cell.ship_type_id() {
            CellStatus::Ship
        } else {
            CellStatus::Empty
        }
    }
}

impl Dimensional for PreGame {
    fn width(&self) -> Dimension {
        self.width
    }

    fn height(&self) -> Dimension {
        self.height
    }
}

impl ShipTypeContainer for PreGame {
    fn ship_types(&self) -> Vec<Arc<ShipType>> {
        self.ship_types.clone()
    }
}

#[cfg(test)]
mod test {
    use common::CellStatus;
    use common::Dimensional;
    use common::Orientation::*;
    use common::Player::*;
    use common::ShipType;
    use common::ShipTypeContainer;
    use pregame::PreGame;
    use results::GameError::IllegalDimensions;
    use results::PlaceError::*;
    use results::GameStartError::*;
    use results::ShipTypeError::*;
    use std::sync::Arc;

    #[test]
    fn constructor_should_check_dimensions() {
        assert_eq!(Err(IllegalDimensions), PreGame::new(0, 0));
        assert_eq!(Err(IllegalDimensions), PreGame::new(0, 5));
        assert_eq!(Err(IllegalDimensions), PreGame::new(5, 0));
        assert!(PreGame::new(2, 2).is_ok());
    }

    #[test]
    fn should_return_dimensions() {
        let game = PreGame::new(2, 3).unwrap();
        assert_eq!(2, game.width());
        assert_eq!(3, game.height());
    }

    #[test]
    fn should_allow_adding_ship_types() {
        let mut game = PreGame::new(3, 3).unwrap();
        assert_eq!(0, game.ship_types().len());

        let ship_type = game.add_ship_type("Corvette", 2).unwrap();
        assert_eq!(1, game.ship_types().len());

        let types = game.ship_types();
        let st = types.first().unwrap();
        assert_eq!(ship_type, *st);
        assert_eq!("Corvette", st.name());
        assert_eq!(2, st.length());
    }

    #[test]
    fn should_disallow_zero_length_ship_types() {
        let mut game = PreGame::new(3, 3).unwrap();

        assert_eq!(Err(IllegalShipLength), game.add_ship_type("Jetski", 0));
    }

    #[test]
    fn should_disallow_too_long_ship_types() {
        let mut game = PreGame::new(3, 3).unwrap();

        game.add_ship_type("Submarine", 1).unwrap();
        assert_eq!(Err(ShipTooLongForBattlefield), game.add_ship_type("Battleship", 4));
    }

    #[test]
    fn should_allow_placing_ships() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette = game.add_ship_type("Corvette", 2).unwrap();

        assert_eq!(Ok(()), game.place_ship(P1, &corvette, 0, 0, Horizontal));
        assert_eq!(Ok(()), game.place_ship(P2, &corvette, 0, 0, Vertical));
    }

    #[test]
    fn should_disallow_placing_ships_twice() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette = game.add_ship_type("Corvette", 2).unwrap();

        assert_eq!(Ok(()), game.place_ship(P1, &corvette, 0, 0, Horizontal));
        assert_eq!(Err(AlreadyPlaced), game.place_ship(P1, &corvette, 0, 1, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_of_unknown_type() {
        let mut game = PreGame::new(3, 3).unwrap();
        let jetski = game.add_ship_type("Jetski", 1).unwrap();
        let car = Arc::new(ShipType::new(0, "Car", 1));
        let fake_jetski = Arc::new(ShipType::new(0, "Jetski", 1));

        assert_eq!(Err(UnknownShipType), game.place_ship(P1, &car, 0, 0, Horizontal));
        assert_eq!(jetski, fake_jetski);
        assert_eq!(Ok(()), game.place_ship(P1, &fake_jetski, 0, 0, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_out_of_bounds() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette = game.add_ship_type("Corvette", 2).unwrap();

        assert_eq!(Err(OutOfBounds), game.place_ship(P1, &corvette, 2, 0, Horizontal));
        assert_eq!(Err(OutOfBounds), game.place_ship(P1, &corvette, 0, 2, Vertical));
        assert_eq!(Ok(()), game.place_ship(P1, &corvette, 1, 0, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_on_top_of_each_other() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette = game.add_ship_type("Corvette", 2).unwrap();
        let frigate = game.add_ship_type("Frigate", 2).unwrap();

        assert_eq!(Ok(()), game.place_ship(P2, &corvette, 0, 0, Horizontal));
        assert_eq!(Err(CellOccupied), game.place_ship(P2, &frigate, 1, 0, Vertical));
    }

    #[test]
    fn should_not_start_when_no_ships_placed() {
        let mut game = PreGame::new(2, 2).unwrap();
        let _ = game.add_ship_type("Corvette", 1).unwrap();

        if let Err((_, NoShipsPlaced)) = game.start() {
            // ok
        } else {
            unreachable!()
        }
    }

    #[test]
    fn should_not_start_when_not_all_ships_placed() {
        let mut game = PreGame::new(2, 2).unwrap();
        let submarine = game.add_ship_type("Submarine", 1).unwrap();
        let corvette = game.add_ship_type("Corvette", 2).unwrap();

        game.place_ship(P1, &submarine, 0, 0, Horizontal).unwrap();
        game.place_ship(P2, &submarine, 0, 0, Horizontal).unwrap();
        game.place_ship(P1, &corvette, 0, 1, Horizontal).unwrap();

        if let Err((_, NotAllShipsPlaced)) = game.start() {
            // ok
        } else {
            unreachable!()
        }
    }

    #[test]
    fn should_start_game() {
        let mut game = PreGame::new(2, 2).unwrap();
        let submarine = game.add_ship_type("Submarine", 1).unwrap();
        game.place_ship(P1, &submarine, 0, 0, Horizontal).unwrap();
        game.place_ship(P2, &submarine, 0, 0, Horizontal).unwrap();

        if let Ok(_) = game.start() {
            // ok
        } else {
            unreachable!()
        }
    }

    #[test]
    fn can_get_cell_status() {
        let mut game = PreGame::new(2, 2).unwrap();
        let submarine = game.add_ship_type("Submarine", 1).unwrap();

        assert_eq!(CellStatus::Empty, game.get_cell(P1, 0, 0));
        game.place_ship(P1, &submarine, 0, 0, Horizontal).unwrap();
        assert_eq!(CellStatus::Ship, game.get_cell(P1, 0, 0));
    }
}
