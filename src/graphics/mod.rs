//! Graphics rendering api

use std::{fmt, sync::Arc};
use crate::{Token, Connection};

#[cfg(all(feature = "cairo", feature = "render"))]
pub mod cairo;
#[cfg(all(feature = "cairo", not(feature = "render")))]
pub mod cairo;
#[cfg(all(feature = "render", not(feature = "cairo")))]
mod cairo;

#[cfg(feature = "render")]
pub use crate::render::{context, context::Context};

/// Text extent width and height
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextExtent(usize, usize);

impl TextExtent {
	/// Get width of the text
	#[inline]
	pub fn width(&self) -> usize {
		self.0
	}
	
	/// Get the height of the text
	#[inline]
	pub fn height(&self) -> usize {
		self.1
	}
	
	/// Get the width and height of the text
	#[inline]
	pub fn dimensions(&self) -> (usize, usize) {
		(self.0, self.1)
	}
}

enum SurfaceType {
	Window(cairo::Surface),
	Buffer(cairo::Surface, cairo::SurfaceContext)
}

/// A surface for drawing graphics
pub struct Surface(SurfaceType);

impl Surface {
	/// Get a window surface from a `Connection`
	pub fn window(connect: &Connection, window: &Token, dimensions: (i32, i32)) -> Option<Self> {
		cairo::surface(connect, window, dimensions).map(|s| Surface{ 0: SurfaceType::Window(s) })
	}
	
	/// Get a buffer surface from a `Connection`
	pub fn buffer(connect: &Connection, window: &Token, dimensions: (i32, i32)) -> Option<Self> {
		cairo::surface_buffer(connect, window, dimensions).map(|s| Surface{ 0: SurfaceType::Buffer(s.0, s.1) })
	}
	
	/// Render graphics to surface using context
	pub fn render(&self, cx: &Context) {
		use SurfaceType::*;
		match &self.0 {
			Window(s) | Buffer(s, _) => { cairo::render(cx, None, s); }
		}
	}
	
	/// Use to copy a buffer surface to a window or another buffer
	pub fn copy(&mut self, other: &Self) {
		use SurfaceType::*;
		match (&self.0, &other.0) {
			(Window(s1), Buffer(s2, _)) |
			(Buffer(s1, _), Window(s2)) |
			(Buffer(s1, _), Buffer(s2, _)) => {
				let cr = cairo::Context::new(s1);
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
	
	/// Get text extent
	pub fn text_extent<T>(&self, text: T, cx: Option<&Context>) -> TextExtent
		where T: AsRef<str> {
		use SurfaceType::*;
		match &self.0 {
			Window(s) | Buffer(s, _) => {
				let mut cr = cairo::Context::new(s);
				if let Some(cx) = cx {
					cr = cairo::render(cx, Some(cr), s);
				}
				let e = cr.text_extents(text.as_ref());
				TextExtent { 0: e.width as usize, 1: e.height as usize }
			}
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
	
	fn inner_surface(&self) -> &cairo::Surface {
		use SurfaceType::*;
		match &self.0 {
			Window(s) | Buffer(s, _) => s
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

impl fmt::Debug for Surface {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Surface")
	}
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

/// A surface for drawing an image. This uses an `Arc` internally
/// so is only as expensive to clone as an `Arc`
#[derive(Clone, Debug)]
#[cfg(feature = "render")]
pub struct ImageSurface(Arc<cairo::ImageSurface>, u32, u32);

#[cfg(feature = "render")]
unsafe impl Send for ImageSurface {}
#[cfg(feature = "render")]
unsafe impl Sync for ImageSurface {}

#[cfg(feature = "render")]
impl ImageSurface {
	/// Get an image surface from image data
	pub fn from(data: Vec<u8>, format: context::ImageFormat,
		width: u32, height: u32) -> Option<Self> {
		cairo::image_surface(data, format, width, height).map(|i| ImageSurface { 0: Arc::new(i), 1: width, 2: height })
	}
	
	/// Get the image width
	pub fn width(&self) -> u32 {
		self.1
	}
	
	/// Get the image height
	pub fn height(&self) -> u32 {
		self.2
	}
	
	/// Get the image width and height
	pub fn dimensions(&self) -> (u32, u32) {
		(self.1, self.2)
	}
}

impl std::hash::Hash for ImageSurface {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		Arc::as_ptr(&self.0).hash(state)
	}
}

impl PartialEq for ImageSurface {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.0, &other.0)
	}
}

impl Eq for ImageSurface {}
