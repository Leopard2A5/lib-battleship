//! This is the documentation for the `lib-battleship` crate.
//!
//! It implements the core logic for every game of battleship.

pub mod pregame;
pub mod game;
pub mod results;
pub mod common;

mod battlefield;

pub type Dimension = usize;
pub type ShipTypeId = usize;
