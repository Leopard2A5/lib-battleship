//! This is the documentation for the `lib-battleship` crate.
//!
//! It implements the core logic for every game of battleship.

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
