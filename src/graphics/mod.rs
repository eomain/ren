//! Graphics rendering api

use std::sync::Arc;
use crate::{Token, Connection};

#[cfg(all(feature = "cairo", feature = "render"))]
pub mod cairo;
#[cfg(all(feature = "cairo", not(feature = "render")))]
pub mod cairo;
#[cfg(all(feature = "render", not(feature = "cairo")))]
mod cairo;

#[cfg(feature = "render")]
pub use crate::render::context;
#[cfg(feature = "render")]
pub use self::cairo::{State, Surface};

/// Get a window surface from a `Connection`
#[cfg(feature = "render")]
pub fn surface(connect: &mut crate::Connection, window: &Token,
    dimensions: (i32, i32)) -> Option<Surface>
{
    cairo::surface(connect, window, dimensions)
}

#[derive(Clone, Debug)]
#[cfg(feature = "render")]
pub struct ImageSurface(Arc<cairo::ImageSurface>);

#[cfg(feature = "render")]
unsafe impl Send for ImageSurface {}
#[cfg(feature = "render")]
unsafe impl Sync for ImageSurface {}

/// Get an image surface from image data
#[cfg(feature = "render")]
pub fn image_surface(data: Vec<u8>, format: context::ImageFormat,
	width: u32, height: u32) -> Option<ImageSurface> {
	cairo::image_surface(data, format, width, height).map(|i| ImageSurface { 0: Arc::new(i) })
}

/// Render graphics to surface using context
#[cfg(feature = "render")]
pub fn render(cx: &context::Context, surface: &Surface,
    mut state: Option<State>) -> Option<State>
{
    cairo::render(cx, surface, state)
}
