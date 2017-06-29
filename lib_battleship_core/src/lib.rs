pub trait Cell {
	fn shot(&self) -> bool;
}

pub trait Battlefield {

	fn cell(
		x: u8,
		y: u8
	) -> Cell;

}

#[derive(Debug, Copy)]
pub enum ShotOutcome {
	Miss,
	Hit,
	WinningHit,
}

pub fn shoot<BF: Battlefield>(
	bf: &BF,
	x: u8,
	y: u8,
) -> ShotOutcome {
	ShotOutcome::Miss
}
