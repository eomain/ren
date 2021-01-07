//! Graphics rendering api

use crate::{Token, Connection};

#[cfg(all(feature = "cairo", feature = "render"))]
pub mod cairo;
#[cfg(all(feature = "cairo", not(feature = "render")))]
pub mod cairo;
#[cfg(all(feature = "render", not(feature = "cairo")))]
mod cairo;

#[cfg(feature = "render")]
pub use mirage::context;
#[cfg(feature = "render")]
pub use self::cairo::{State, Surface};

/// Get a window surface from a `Connection`
#[cfg(feature = "render")]
pub fn surface(connect: &mut crate::Connection, window: &Token,
    dimensions: (i32, i32)) -> Option<Surface>
{
    cairo::surface(connect, window, dimensions)
}

/// Render graphics to surface using context
#[cfg(feature = "render")]
pub fn render(cx: &context::Context, surface: &Surface,
    mut state: Option<State>) -> Option<State>
{
    cairo::render(cx, surface, state)
}
