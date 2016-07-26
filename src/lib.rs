//! # The Battleship library
//! This library implements the core functions of every battleship game.
//! There's no UI code in here, it's strictly state and logic.

pub mod battlefield;
pub mod cell;
pub mod ship;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
