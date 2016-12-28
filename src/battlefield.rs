use results::GameError::{self, IllegalDimensions};
use cell::Cell;
use super::Dimension;

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

    pub fn width(&self) -> Dimension {
        self.cells.first().unwrap().len()
    }

    pub fn height(&self) -> Dimension {
        self.cells.len()
    }

    pub fn get_cell(
        &self,
        x: Dimension,
        y: Dimension,
    ) -> Option<&Cell> {
        self.cells.get(y)
            .and_then(|row| row.get(x))
    }

    pub fn get_mut_cell(
        &mut self,
        x: Dimension,
        y: Dimension,
    ) -> Option<&mut Cell> {
        self.cells.get_mut(y)
            .and_then(|row| row.get_mut(x))
    }
}

#[cfg(test)]
mod test {
    use battlefield::Battlefield;

    #[test]
    fn should_give_out_cell_references() {
        let bf = Battlefield::new(2, 2).unwrap();

        assert!(bf.get_cell(0, 0).is_some());
        assert!(bf.get_cell(0, 2).is_none());
    }

    #[test]
    fn should_return_width() {
        let bf = Battlefield::new(2, 3).unwrap();

        assert_eq!(2, bf.width());
        assert_eq!(3, bf.height());
    }
}
