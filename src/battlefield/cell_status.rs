#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellStatus {
    Empty,
    Miss,
    Ship,
    Hit,
}
