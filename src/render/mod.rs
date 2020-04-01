
extern crate mirage;

pub use render::mirage::object::{
    Point,
    Line,
    Rect,
    Translate
};

pub use render::mirage::object::text::Text as Font;

pub use render::mirage::surface::{
    Object,
    Surface
};

pub(crate) mod xcb;
