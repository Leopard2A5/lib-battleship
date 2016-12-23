use errors::GameError::{self, IllegalDimensions};
use cell::Cell;
use game::Dimension;

#[derive(Clone, PartialEq, Debug)]
pub struct Battlefield {
    cells: Vec<Vec<Cell>>,
}

impl Battlefield {
    pub fn new(
        width: Dimension,
        height: Dimension,
    ) -> Result<Battlefield, GameError> {
        if width < 2 || height < 2 {
            Err(IllegalDimensions)
        } else {
            Ok(Battlefield {
                cells: Battlefield::init_cells(width, height),
            })
        }
    }

    fn init_cells(
        width: Dimension,
        height: Dimension,
    ) -> Vec<Vec<Cell>> {
        let mut ret = Vec::new();
        for _ in 0..height {
            let mut line = Vec::new();
            for _ in 0..width {
                line.push(Cell::new())
            }
            ret.push(line);
        }
        ret
    }
}
