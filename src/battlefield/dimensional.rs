use ::Dimension;

pub trait Dimensional {
    /// Returns the number of columns for the battlefields in this `PreGame`.
    fn width(&self) -> Dimension;

    /// Returns the number of lines for the battlefields in this `PreGame`.
    fn height(&self) -> Dimension;
}
