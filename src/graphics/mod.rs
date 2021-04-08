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
pub use crate::render::{context, context::Context};

enum SurfaceType {
	Window(cairo::Surface),
	Buffer(cairo::Surface, cairo::SurfaceContext)
}

/// A surface for drawing graphics
pub struct Surface(SurfaceType);

impl Surface {
	/// Get a window surface from a `Connection`
	pub fn window(connect: &mut Connection, window: &Token, dimensions: (i32, i32)) -> Option<Self> {
		cairo::surface(connect, window, dimensions).map(|s| Surface{ 0: SurfaceType::Window(s) })
	}
	
	/// Get a buffer surface from a `Connection`
	pub fn buffer(connect: &mut Connection, window: &Token, dimensions: (i32, i32)) -> Option<Self> {
		cairo::surface_buffer(connect, window, dimensions).map(|s| Surface{ 0: SurfaceType::Buffer(s.0, s.1) })
	}
	
	/// Render graphics to surface using context
	pub fn render(&self, cx: &context::Context) {
		use SurfaceType::*;
		match &self.0 {
			Window(s) | Buffer(s, _) => { cairo::render(cx, s, None); }
		}
	}
	
	/// Use to copy a buffer surface to a window or another buffer
	pub fn copy(&mut self, other: &Self) {
		use SurfaceType::*;
		match (&self.0, &other.0) {
			(Window(s1), Buffer(s2, _)) |
			(Buffer(s1, _), Window(s2)) |
			(Buffer(s1, _), Buffer(s2, _)) => {
				let cr = cairo::Context::new(&s1);
				cr.set_source_surface(s2, 0.0, 0.0);
				cr.paint();
			},
			_ => ()
		}
	}
	
	/// Update the surface
	pub fn update(&mut self, (width, height): (i32, i32)) {
		use SurfaceType::*;
		match &self.0 {
			Window(s) => {
				s.set_size(width, height);
			},
			_ => ()
		}
	}
	
	/// Check if this is a window surface
	pub fn is_window_surface(&self) -> bool {
		use SurfaceType::*;
		match &self.0 {
			Window(s) => true,
			_ => false
		}
	}
}

#[cfg(feature = "cairo")]
impl Surface {
	/// Create a new cairo context from the surface
	#[inline]
	pub fn create_cairo_context(&self) -> cairo::Context {
		use SurfaceType::*;
		match &self.0 {
			Window(s) | Buffer(s, _) => cairo::Context::new(s)
		}
	}
	
	/// Returns the surface as a cairo surface
	#[inline]
	pub fn as_cairo_surface(&self) -> &cairo::Surface {
		use SurfaceType::*;
		match &self.0 {
			Window(s) | Buffer(s, _) => &s
		}
	}
}

/// A surface for drawing an image. This uses an `Arc` internally
/// so is only as expensive to clone as an `Arc`
#[derive(Clone, Debug)]
#[cfg(feature = "render")]
pub struct ImageSurface(Arc<cairo::ImageSurface>);

#[cfg(feature = "render")]
unsafe impl Send for ImageSurface {}
#[cfg(feature = "render")]
unsafe impl Sync for ImageSurface {}

#[cfg(feature = "render")]
impl ImageSurface {
	/// Get an image surface from image data
	pub fn from(data: Vec<u8>, format: context::ImageFormat,
		width: u32, height: u32) -> Option<Self> {
		cairo::image_surface(data, format, width, height).map(|i| ImageSurface { 0: Arc::new(i) })
	}
}

impl PartialEq for ImageSurface {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.0, &other.0)
	}
}
