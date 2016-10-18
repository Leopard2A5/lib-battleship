#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlaceResultErr {
    OutOfBounds,
    CellOccupied,
}

/// The possible results of placing a ship on the battlefield.
pub type PlaceResult = Result<(), PlaceResultErr>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotResultOk {
    Miss,
    Hit,
    ShipDestroyed,
    WinningShot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShotResultErr {
    OutOfBounds,
    AlreadyShot,
}

/// The possible outcomes of shooting at a cell on the battlefield.
pub type ShotResult = Result<ShotResultOk, ShotResultErr>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CommonError {
    IllegalDimensions,
}
