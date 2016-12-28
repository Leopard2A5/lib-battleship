#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn next(&self) -> Player {
        match *self {
            Player::P1 => {
                Player::P2
            },
            Player::P2 => {
                Player::P1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Player::*;

    #[test]
    fn should_alternate_between_players() {
        assert_eq!(P2, P1.next());
        assert_eq!(P1, P2.next());
    }
}
