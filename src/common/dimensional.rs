use ::Dimension;

/// Denotes a struct that has a width and height of type `Dimension`.
pub trait Dimensional {
    /// Returns the width of this object.
    fn width(&self) -> Dimension;

    /// Returns the height of this object.
    fn height(&self) -> Dimension;
}
