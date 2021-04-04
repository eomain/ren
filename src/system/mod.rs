
use crate::{
	Stat, Data, WindowCommand,
	event::Event,
	context::{xcb, WindowContext, ConnectionError}
};

/// The kind of windowing system
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemType {
	Xcb
}

impl Default for SystemType {
	#[cfg(target_family = "unix")]
	fn default() -> Self {
		SystemType::Xcb
	}
}

pub enum Window {
	Xcb(xcb::Window)
}

impl Window {
	pub fn event(&self) -> Option<Event> {
		use Window::*;
		match self {
			Xcb(w) => w.event()
		}
	}
	
	pub fn poll(&self) -> Option<Event> {
		use Window::*;
		match self {
			Xcb(w) => w.poll()
		}
	}
	
	pub fn stat(&self, stat: Stat) -> Option<Data> {
		use Window::*;
		match self {
			Xcb(w) => w.stat(stat)
		}
	}
	
	pub fn window(&self, command: &WindowCommand) {
		use Window::*;
		match self {
			Xcb(w) => w.window(command)
		}
	}
}

pub enum SystemConnection {
	Xcb(xcb::Connection)
}

impl SystemConnection {
	pub fn new(ty: SystemType) -> Result<Self, Option<ConnectionError>>
	{
		match ty {
			SystemType::Xcb  => {
				Ok(SystemConnection::Xcb(xcb::Connection::open()?))
			}
		}
	}
	
	#[inline]
	pub fn create_window(&self) -> Window {
		match self {
			SystemConnection::Xcb(c) => Window::Xcb(c.create_window())
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
