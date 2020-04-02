
pub use mirage::*;
pub use mirage::object::{
    Point,
    Line,
    Rect,
    Translate
};

pub use mirage::object::text::Text as Font;

pub use mirage::surface::{
    Primitive,
    Object,
    Surface
};

pub(crate) mod xcb;
