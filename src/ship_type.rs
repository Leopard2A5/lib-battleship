use super::Dimension;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ShipType {
    name: &'static str,
    length: Dimension,
}

impl ShipType {
    pub fn new(
        name: &'static str,
        length: Dimension,
    ) -> Self {
        ShipType {
            name: name,
            length: length,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn length(&self) -> Dimension {
        self.length
    }
}

#[cfg(test)]
mod test {
    use ship_type::ShipType;

    #[test]
    fn constructor_should_work() {
        let typ = ShipType::new("foo", 5);
        assert_eq!("foo", typ.name());
        assert_eq!(5, typ.length());
    }
}
