
pub mod xcb;

use crate::{Stat, Data, WindowCommand, event::Event};

/// A connection error with the windowing system
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionError {
	
}

pub trait WindowContext {

	fn event(&self) -> Option<Event>;

	fn poll(&self) -> Option<Event>;

	fn stat(&self, _: Stat) -> Option<Data>;

	fn window(&self, _: &WindowCommand);

	fn update(&self);

}
