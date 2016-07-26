//! # The Battleship library
//! This library implements the core functions of every battleship game.
//! There's no UI code in here, it's strictly state and logic.
//! # Examples
//! ```
//! use lib_battleship::battlefield::Battlefield;
//! let bf = Battlefield::new();
//! ```

pub mod battlefield;
pub mod cell;
pub mod ship;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
