use ship_type::ShipType;
use player::Player::{self, P1};
use super::Dimension;
use super::ShipTypeId;
use std::mem;

#[derive(PartialEq, Debug)]
pub struct ShipStatus {
    status_p1: Vec<Dimension>,
    status_p2: Vec<Dimension>,
}

impl ShipStatus {
    pub fn new(
        ship_types: &Vec<ShipType>,
    ) -> Self {
        ShipStatus {
            status_p1: ship_types.iter().map(|st| st.length()).collect(),
            status_p2: ship_types.iter().map(|st| st.length()).collect(),
        }
    }

    pub fn get_health(
        &self,
        player: Player,
        ship_type_id: ShipTypeId,
    ) -> Dimension {
        if player == P1 {
            *self.status_p1.get(ship_type_id).unwrap()
        } else {
            *self.status_p2.get(ship_type_id).unwrap()
        }
    }

    pub fn hit(
        &mut self,
        player: Player,
        ship_type_id: ShipTypeId,
    ) -> Dimension {
        if player == P1 {
            let curr_val = *self.status_p1.get(ship_type_id).unwrap();
            mem::replace(&mut self.status_p1[ship_type_id], curr_val - 1);
            *self.status_p1.get(ship_type_id).unwrap()
        } else {
            let curr_val = *self.status_p2.get(ship_type_id).unwrap();
            mem::replace(&mut self.status_p2[ship_type_id], curr_val - 1);
            *self.status_p2.get(ship_type_id).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use ship_type::ShipType;
    use ship_status::ShipStatus;
    use player::Player::*;

    #[test]
    fn should_init_with_length() {
        let types = vec!(ShipType::new("Corvette", 2));
        let status = ShipStatus::new(&types);

        assert_eq!(2, status.get_health(P1, 0));
        assert_eq!(2, status.get_health(P2, 0));
    }

    #[test]
    fn should_reduce_health() {
        let types = vec!(ShipType::new("Corvette", 2));
        let mut status = ShipStatus::new(&types);

        assert_eq!(1, status.hit(P1, 0));
        assert_eq!(1, status.get_health(P1, 0));
        assert_eq!(1, status.hit(P2, 0));
        assert_eq!(1, status.get_health(P2, 0));
    }
}
