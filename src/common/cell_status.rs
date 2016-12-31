
/// The (display) states a cell on the battlefield can have.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellStatus {
    Empty,
    Miss,
    Ship,
    Hit,
}
