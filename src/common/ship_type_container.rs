use common::ShipType;

/// Denotes a type that holds ship types.
pub trait ShipTypeContainer {
    /// Returns a copy of the list of ship types.
    fn ship_types(&self) -> Vec<ShipType>;
}
