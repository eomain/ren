
use crate::{
	Stat, Data, WindowCommand,
	event::Event,
	context::{WindowContext, ConnectionError}
};

#[cfg(target_family = "unix")]
use crate::context::xcb;

/// The kind of windowing system
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemType {
	#[cfg(target_family = "unix")]
	Xcb
}

impl Default for SystemType {
	#[cfg(target_family = "unix")]
	fn default() -> Self {
		SystemType::Xcb
	}
}

pub enum Window {
	#[cfg(target_family = "unix")]
	Xcb(xcb::Window)
}

impl Window {
	#[cfg(target_family = "unix")]
	pub fn event(&self) -> Option<Event> {
		use Window::*;
		match self {
			Xcb(w) => w.event()
		}
	}
	
	#[cfg(target_family = "unix")]
	pub fn poll(&self) -> Option<Event> {
		use Window::*;
		match self {
			Xcb(w) => w.poll()
		}
	}
	
	#[cfg(target_family = "unix")]
	pub fn stat(&self, stat: Stat) -> Option<Data> {
		use Window::*;
		match self {
			Xcb(w) => w.stat(stat)
		}
	}
	
	#[cfg(target_family = "unix")]
	pub fn window(&self, command: &WindowCommand) {
		use Window::*;
		match self {
			Xcb(w) => w.window(command)
		}
	}
}

enum SystemConnection {
	#[cfg(target_family = "unix")]
	Xcb(xcb::Connection)
}

impl SystemConnection {
	#[cfg(target_family = "unix")]
	fn new(ty: SystemType) -> Result<Self, Option<ConnectionError>>
	{
		match ty {
			SystemType::Xcb => {
				Ok(SystemConnection::Xcb(xcb::Connection::open()?))
			}
		}
	}
	
	#[inline]
	#[cfg(target_family = "unix")]
	fn create_window(&self) -> Window {
		match self {
			SystemConnection::Xcb(c) => Window::Xcb(c.into())
		}
	}
}

pub struct System {
	ty: SystemType,
	connection: SystemConnection
}

impl System {
	pub fn new(ty: SystemType) -> Result<Self, Option<ConnectionError>> {
		Ok(Self {
			ty,
			connection: SystemConnection::new(ty)?
		})
	}
	
	pub fn create_window(&self) -> Window {
		self.connection.create_window()
	}
}
