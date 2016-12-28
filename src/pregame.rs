//! Everything you need to set up a game of battleship.

use battlefield::Battlefield;
use results::GameError;
use results::GameStartError;
use results::GameStartError::*;
use results::PlaceError;
use results::PlaceError::*;
use results::ShipTypeError;
use results::ShipTypeError::*;
use game::Game;
use common::Orientation;
use common::Player;
use common::Player::*;
use battlefield::ShipType;
use std::collections::HashSet;
use std::cmp::max;
use super::Dimension;
use super::ShipTypeId;

#[derive(PartialEq, Debug)]
/// foo
pub struct PreGame {
    width: Dimension,
    height: Dimension,
    ship_types: Vec<ShipType>,
    placed_ships: HashSet<(Player, ShipTypeId)>,
    battlefields: Vec<Battlefield>,
}

/// bar
impl PreGame {
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

    pub fn start(self) -> Result<Game, (Self, GameStartError)> {
        if self.placed_ships.len() == 0 {
            Err((self, NoShipsPlaced))
        } else if self.placed_ships.len() != (2 * self.ship_types.len()) {
            Err((self, NotAllShipsPlaced))
        } else {
            Ok(Game::new(self.ship_types, self.battlefields))
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
    ) -> Result<ShipTypeId, ShipTypeError> {
        if length == 0 {
            Err(IllegalShipLength)
        } else if length > max(self.width(), self.height()) {
            Err(ShipTooLongForBattlefield)
        } else {
            let typ = ShipType::new(name, length);
            self.ship_types.push(typ);
            Ok(self.ship_types.len() - 1)
        }
    }

    pub fn ship_types(&self) -> Vec<ShipType> {
        self.ship_types.clone()
    }

    pub fn place_ship(
        &mut self,
        player: Player,
        ship_type_id: ShipTypeId,
        x: Dimension,
        y: Dimension,
        orientation: Orientation,
    ) -> Result<(), PlaceError> {
        let ship_type = self.ship_types
            .get(ship_type_id)
            .ok_or(UnknownShipTypeId)?.clone();
        self.assert_ship_not_yet_placed(player, ship_type_id)?;
        self.assert_ship_placement_in_bounds(&ship_type, x, y, orientation)?;
        let affected_cell_coords = self.get_affected_cell_coords(&ship_type, x, y, orientation);
        self.assert_cells_free(player, &affected_cell_coords)?;

        self.do_place_ship(player, ship_type_id, &affected_cell_coords);
        self.placed_ships.insert((player, ship_type_id));
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
}

#[cfg(test)]
mod test {
    use pregame::PreGame;
    use results::GameError::IllegalDimensions;
    use common::Player::*;
    use results::PlaceError::*;
    use results::GameStartError::*;
    use results::ShipTypeError::*;
    use common::Orientation::*;

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

        let ship_type_id = game.add_ship_type("Corvette", 2).unwrap();
        assert_eq!(0, ship_type_id);
        assert_eq!(1, game.ship_types().len());

        let types = game.ship_types();
        let st = types.first().unwrap();
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
        let corvette_id = game.add_ship_type("Corvette", 2).unwrap();

        assert_eq!(Ok(()), game.place_ship(P1, corvette_id, 0, 0, Horizontal));
        assert_eq!(Ok(()), game.place_ship(P2, corvette_id, 0, 0, Vertical));
    }

    #[test]
    fn should_disallow_placing_ships_twice() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2).unwrap();

        assert_eq!(Ok(()), game.place_ship(P1, corvette_id, 0, 0, Horizontal));
        assert_eq!(Err(AlreadyPlaced), game.place_ship(P1, corvette_id, 0, 1, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_out_of_bounds() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2).unwrap();

        assert_eq!(Err(OutOfBounds), game.place_ship(P1, corvette_id, 2, 0, Horizontal));
        assert_eq!(Err(OutOfBounds), game.place_ship(P1, corvette_id, 0, 2, Vertical));
        assert_eq!(Ok(()), game.place_ship(P1, corvette_id, 1, 0, Horizontal));
    }

    #[test]
    fn should_disallow_placing_ships_on_top_of_each_other() {
        let mut game = PreGame::new(3, 3).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2).unwrap();
        let frigate_id = game.add_ship_type("Frigate", 2).unwrap();

        assert_eq!(Ok(()), game.place_ship(P2, corvette_id, 0, 0, Horizontal));
        assert_eq!(Err(CellOccupied), game.place_ship(P2, frigate_id, 1, 0, Vertical));
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
        let submarine_id = game.add_ship_type("Submarine", 1).unwrap();
        let corvette_id = game.add_ship_type("Corvette", 2).unwrap();

        game.place_ship(P1, submarine_id, 0, 0, Horizontal).unwrap();
        game.place_ship(P2, submarine_id, 0, 0, Horizontal).unwrap();
        game.place_ship(P1, corvette_id, 0, 1, Horizontal).unwrap();

        if let Err((_, NotAllShipsPlaced)) = game.start() {
            // ok
        } else {
            unreachable!()
        }
    }

    #[test]
    fn should_start_game() {
        let mut game = PreGame::new(2, 2).unwrap();
        let submarine_id = game.add_ship_type("Submarine", 1).unwrap();
        game.place_ship(P1, submarine_id, 0, 0, Horizontal).unwrap();
        game.place_ship(P2, submarine_id, 0, 0, Horizontal).unwrap();

        if let Ok(_) = game.start() {
            // ok
        } else {
            unreachable!()
        }
    }

}
