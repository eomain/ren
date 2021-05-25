
pub mod keyboard;

use keyboard::{Modifier, Modifiers, Mapping};
use super::{Event, InputEvent, Position};

pub use keyboard::{KeyCode, KeyMap};

/// Keyboard input data
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyInput {
	code: KeyCode,
	mods: Option<Modifiers>,
	map: Mapping
}

impl KeyInput {
	pub(crate) fn new(code: KeyCode, mods: Option<Modifiers>, map: Mapping) -> Self {
		Self {
			code,
			mods,
			map
		}
	}

	/// Return the keycode
	pub fn code(&self) -> KeyCode {
		self.code
	}

	/// The key modifiers associated with the input
	pub fn modifiers(&self) -> Option<&[Modifier]> {
		self.mods
	}
}

/// The type of Key event.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyEvent {
	/// A Key press has occured.
	Press(KeyInput),
	/// A Key release has occured.
	Release(KeyInput)
}

impl From<KeyEvent> for InputEvent {
	fn from(k: KeyEvent) -> Self
	{
		InputEvent::Key(k)
	}
}

impl From<KeyEvent> for Event {
	fn from(k: KeyEvent) -> Self
	{
		Event::Input(k.into())
	}
}

/// The mouse input type
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseInput {
	/// Left Mouse button
	Left,
	/// Right Mouse button
	Right,
	/// Middle Mouse button
	Middle,
	/// Mouse scroll up
	ScrollUp,
	/// Mouse scroll down
	ScrollDown,
	/// An unknown button
	Unknown(Option<u8>)
}

/// Mouse event data
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseData {
	input: MouseInput,
	position: Position
}

impl MouseData {
	pub(crate) fn new(input: MouseInput, position: Position) -> Self {
		Self {
			input,
			position
		}
	}
	
	/// The mouse button that triggered the event
	#[inline]
	pub fn input(&self) -> MouseInput {
		self.input
	}
	
	/// The position of the event
	#[inline]
	pub fn position(&self) -> Position {
		self.position
	}
}

/// The type of mouse event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseEvent {
	/// A Mouse press has occured.
	Press(MouseData),
	/// A Mouse release has occured.
	Release(MouseData),
	/// A Mouse movement has occured.
	Move(Position),
	/// The Mouse pointer has entered the Window
	Enter(Position),
	/// The Mouse pointer has left the Window
	Leave(Position)
}

impl From<MouseEvent> for InputEvent {
	fn from(m: MouseEvent) -> Self
	{
		InputEvent::Mouse(m)
	}
}

impl From<MouseEvent> for Event {
	fn from(m: MouseEvent) -> Self
	{
		Event::Input(m.into())
	}
}
