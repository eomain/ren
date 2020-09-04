
use super::{
    Position,
    Dimension
};

/// A map of an area of the display
/// that needs to be updated.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExposeMap {
    /// The position within the display.
    pos: Position,
    /// The dimensions of the map.
    dim: Dimension
}

impl ExposeMap {

    pub fn new(pos: Position, dim: Dimension) -> Self
    {
        Self {
            pos,
            dim
        }
    }
}

impl ExposeMap {

    pub fn position(&self) -> Position
    {
        self.pos
    }

    pub fn dimension(&self) -> Dimension
    {
        self.dim
    }
}
