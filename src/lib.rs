//! This is the documentation for the `lib-battleship` crate.
//!
//! It implements the core logic for every game of battleship.

pub mod pregame;
pub mod game;
pub mod results;
pub mod common;

mod battlefield;

/// Dimension type for battleship.
pub type Dimension = usize;

/// The type of ship type IDs.
pub type ShipTypeId = usize;
