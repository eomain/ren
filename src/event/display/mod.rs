
use super::{
    Position,
    Dimension
};

/// A map of an area of the window
/// that needs to be redrawn.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Map(pub Position, pub Dimension);
