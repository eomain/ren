//! # Render

//! Contains primatives for draw graphics to the display.

pub(crate) mod xcb;

pub use mirage::*;
pub use mirage::object::{
    Point,
    Line,
    Rect,
    Translate
};

pub use mirage::object::text::Text as Font;
pub use mirage::object::bitmap::Bitmap as Image;

pub use mirage::surface::{
    Primitive,
    Object,
    Surface
};
