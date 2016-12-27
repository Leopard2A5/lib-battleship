#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameError {
    IllegalDimensions,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShipTypeError {
    IllegalShipLength,
    ShipTooLongForBattlefield,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlaceError {
    AlreadyPlaced,
    OutOfBounds,
    UnknownShipTypeId,
    CellOccupied,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameStartError {
    NoShipsPlaced,
    NotAllShipsPlaced,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShootOk {
    Hit,
    Miss,
    Destroyed,
    WinningShot,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShootError {
    NotThisPlayersTurn,
    OutOfBounds,
}
