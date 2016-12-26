use ship_type::ShipType;
use battlefield::Battlefield;
use super::Dimension;

#[derive(PartialEq, Debug)]
pub struct Game {
    ship_types: Vec<ShipType>,
    battlefields: Vec<Battlefield>,
}

impl Game {
    pub fn new(
        ship_types: Vec<ShipType>,
        battlefields: Vec<Battlefield>,
    ) -> Self {
        Game {
            ship_types: ship_types,
            battlefields: battlefields,
        }
    }

    pub fn width(&self) -> Dimension {
        self.battlefields.first().unwrap().width()
    }

    pub fn height(&self) -> Dimension {
        self.battlefields.first().unwrap().height()
    }

    // pub fn shoot(
    //     &mut self,
    //     target_player: Player,
    //     x: Dimension,
    //     y: Dimension,
    // ) -> Result<(), ShootError> {
    //     Ok(())
    // }
}

#[cfg(test)]
mod test {
    use game::Game;
    use ship_type::ShipType;
    use battlefield::Battlefield;

    #[test]
    fn should_return_dimensions() {
        let ship_types = vec!(ShipType::new("Corvette", 2));
        let bf1 = Battlefield::new(2, 3).unwrap();
        let bf2 = bf1.clone();
        let battlefields = vec!(bf1, bf2);

        let game = Game::new(ship_types, battlefields);
        assert_eq!(2, game.width());
        assert_eq!(3, game.height());
    }
}
