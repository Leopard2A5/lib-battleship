use ::Dimension;

/// Represents a ship type in the game.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ShipType {
    name: &'static str,
    length: Dimension,
}

impl ShipType {
    /// Create a new ship type.
    /// # Parameters
    /// * `name` the name of the ship type.
    /// * `length` The length of the ship type.
    pub fn new(
        name: &'static str,
        length: Dimension,
    ) -> Self {
        ShipType {
            name: name,
            length: length,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the length.
    pub fn length(&self) -> Dimension {
        self.length
    }
}

#[cfg(test)]
mod test {
    use super::ShipType;

    #[test]
    fn constructor_should_work() {
        let typ = ShipType::new("foo", 5);
        assert_eq!("foo", typ.name());
        assert_eq!(5, typ.length());
    }
}
