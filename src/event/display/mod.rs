
use super::{Position, Dimension, DisplayEvent};

macro_rules! display_from {
	($t: ty, $i: ident) => {
		impl From<$t> for DisplayEvent {
			fn from(e: $t) -> Self
			{
				DisplayEvent::$i(e)
			}
		}
	}
}

/// A map of an area of the window
/// that needs to be redrawn.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Map(pub Position, pub Dimension);

/// Window focus event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FocusEvent {
	/// The window gained focus
	Gain,
	/// The window lost focus
	Lose
}

display_from!(FocusEvent, Focus);