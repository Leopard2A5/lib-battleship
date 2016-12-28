//! This is the documentation for the `lib-battleship` crate.
//!
//! It implements the core logic for every game of battleship.

pub mod pregame;
pub mod game;
pub mod results;
pub mod player;
pub mod orientation;
pub mod cell_status;

mod battlefield;
mod cell;
mod ship_status;
mod ship_type;

pub type Dimension = usize;
pub type ShipTypeId = usize;
