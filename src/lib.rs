pub mod pregame;
pub mod game;
pub mod errors;
pub mod ship_type;
pub mod player;
pub mod orientation;
pub mod battlefield;
pub mod cell;

pub type Dimension = usize;
pub type ShipTypeId = usize;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
