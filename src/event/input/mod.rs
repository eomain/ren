
use super::{
	Event,
	InputEvent,
	Position
};

/// Keyboard input data
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyCode(pub u16);

impl KeyCode {
	pub(crate) fn new(code: u16) -> Self {
		Self { 0: code }
	}

	/// Return the keycode
	pub fn code(&self) -> u16 {
		self.0
	}

	/// Map the keycode to a keyboard character map
	#[allow(dead_code)]
	fn map(&self) -> KeyMap {
		unimplemented!()
	}
}

/// A mapping of possible keyboard characters.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyMap {
	Esc,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,

	Num0,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,

	Shift,
	Caps,

	Up,
	Down,
	Left,
	Right,

	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	/// An unknown key
	Unknown(u16)
}

/// The type of Key event.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyEvent {
	/// A Key press has occured.
	Press(KeyCode),
	/// A Key release has occured.
	Release(KeyCode)
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
