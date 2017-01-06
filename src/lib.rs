//! This is the documentation for the `lib-battleship` crate. It implements the core logic for
//! every game of battleship.
//!
//! You can set up the game by creating and configuring a `PreGame` object like so:
//!
//! ```
//! use lib_battleship::PreGame;
//! use lib_battleship::common::CellStatus;
//! use lib_battleship::common::Dimensional;
//! use lib_battleship::common::Player::*;
//! use lib_battleship::common::Orientation::*;
//! use lib_battleship::results::ShootOk;
//!
//! // when hard-coding the game's dimensions, use `unwrap()`.
//! // PreGame's constructor makes sure that the battlefield
//! // is no smaller than 2x2.
//! let mut pregame = PreGame::new(10, 10).unwrap();
//!
//! // add ship types
//! // `PreGame` validates that a ship is no shorter
//! // than 1 in length, thus the call to `unwrap()`.
//! let sub = pregame.add_ship_type("Submarine", 1).unwrap();
//! let corvette = pregame.add_ship_type("Corvette", 2).unwrap();
//!
//! // Then each player has to place all their ships on the battlefield. Each player has one
//! // ship of every ship type. PreGame also validates the placement of each ship.
//! pregame.place_ship(P1, &corvette, 0, 0, Horizontal).unwrap();
//! pregame.place_ship(P1, &sub, 9, 9, Horizontal).unwrap();
//!
//! pregame.place_ship(P2, &corvette, 5, 5, Vertical).unwrap();
//! pregame.place_ship(P2, &sub, 3, 7, Horizontal).unwrap();
//!
//! // Display a player's battlefield by iterating over its cells like so:
//! for y in 0..pregame.height() {
//!     for x in 0..pregame.width() {
//!         // PreGame::get_cell only returns Empty or Ship.
//!         let char = match pregame.get_cell(P1, x, y) {
//!             CellStatus::Empty => " ",
//!             CellStatus::Ship => "X",
//!             _ => unreachable!()
//!         };
//!         print!("{}", char);
//!     }
//!     println!("");
//! }
//!
//! // When all ships have been placed, start the game. PreGame::start() checks that all ships
//! // have been placed and will complain if that's not the case.
//! let mut game = pregame.start().unwrap();
//!
//! // From now on, players can take turns shooting at each other's ships. A player can
//! // keep shooting as long as they score hits.
//!
//! // Game validates that it's the shooting player's turn and that they don't shoot out of
//! // bounds, hence the call to unwrap().
//! match game.shoot(P2, 0, 0).unwrap() {
//!   ShootOk::Hit => println!("hit!"),
//!   ShootOk::Miss => println!("miss!"),
//!   ShootOk::Destroyed => println!("ship destroyed!"),
//!   ShootOk::WinningShot => println!("you won!")
//! }
//! ```

pub use self::game::Game;
pub use self::pregame::PreGame;

pub mod common;
pub mod results;

mod battlefield;
mod game;
mod pregame;

/// Dimension type for battleship.
pub type Dimension = usize;

/// The type of ship type IDs.
pub type ShipTypeId = usize;
