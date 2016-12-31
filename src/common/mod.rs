//! Common types, functions, etc.

pub use self::cell_status::CellStatus;
pub use self::dimensional::Dimensional;
pub use self::orientation::Orientation;
pub use self::player::Player;
pub use self::ship_type::ShipType;
pub use self::ship_type_container::ShipTypeContainer;

mod cell_status;
mod dimensional;
mod orientation;
mod player;
mod ship_type;
mod ship_type_container;
