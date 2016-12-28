//! Result types for all operations that can fail.

/// General errors when creating a game.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameError {
    IllegalDimensions,
}

/// Errors concerning ship types.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShipTypeError {
    IllegalShipLength,
    ShipTooLongForBattlefield,
}

/// Errors that can occur whene placing ships.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlaceError {
    AlreadyPlaced,
    OutOfBounds,
    UnknownShipTypeId,
    CellOccupied,
}

/// Possible errors when trying to start playing.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameStartError {
    NoShipsPlaced,
    NotAllShipsPlaced,
}

/// Possible positive outcomes of shooting.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShootOk {
    Hit,
    Miss,
    Destroyed,
    WinningShot,
}

/// Errors that can occur when shooting.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShootError {
    NotThisPlayersTurn,
    OutOfBounds,
}
