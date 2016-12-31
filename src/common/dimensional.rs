use ::Dimension;

pub trait Dimensional {
    /// Returns the width of this object.
    fn width(&self) -> Dimension;

    /// Returns the height of this object.
    fn height(&self) -> Dimension;
}
