
use std::collections::HashSet;

/// Possible keyboard modifiers/modes
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Modifier {
	Ctrl,
	Shift,
	Caps,
	Alt,
	NumLock,
	ScrollLock
}

/// Raw input keycode
pub type KeyCode = u16;

pub type Modifiers = &'static [Modifier];

/// Converts a `KeyCode` into a `KeyMap`
pub(crate) type Mapping = fn(KeyCode) -> Option<KeyMap>;

/// A mapping of possible keyboard characters.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyMap {
	Escape,
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
	PrintScreen,
	ScrollLock,
	Pause,
	Digit0,
	Digit1,
	Digit2,
	Digit3,
	Digit4,
	Digit5,
	Digit6,
	Digit7,
	Digit8,
	Digit9,
	Minus,
	Equal,
	Backspace,
	Space,
	Enter,
	ShiftLeft,
	ShiftRight,
	NumLock,
	CapsLock,
	Tab,
	CtrlLeft,
	CtrlRight,
	AltLeft,
	AltRight,
	ContextMenu,
	ArrowUp,
	ArrowDown,
	ArrowLeft,
	ArrowRight,
	Insert,
	Delete,
	Home,
	End,
	PageUp,
	PageDown,
	BracketLeft,
	BracketRight,
	Semicolon,
	Quote,
	Backquote,
	Backslash,
	IntlBackslash,
	Comma,
	Period,
	Slash,
	KeyA,
	KeyB,
	KeyC,
	KeyD,
	KeyE,
	KeyF,
	KeyG,
	KeyH,
	KeyI,
	KeyJ,
	KeyK,
	KeyL,
	KeyM,
	KeyN,
	KeyO,
	KeyP,
	KeyQ,
	KeyR,
	KeyS,
	KeyT,
	KeyU,
	KeyV,
	KeyW,
	KeyX,
	KeyY,
	KeyZ,
	Numpad0,
	Numpad1,
	Numpad2,
	Numpad3,
	Numpad4,
	Numpad5,
	Numpad6,
	Numpad7,
	Numpad8,
	Numpad9,
	NumpadDivide,
	NumpadMultiply,
	NumpadSubtract,
	NumpadAdd,
	NumpadDecimal,
	NumpadEnter,
	AudioVolumeMute,
	AudioVolumeDown,
	AudioVolumeUp
}

trait KeyString {
	fn as_str(&self, _: KeyMap, _: Option<&HashSet<Modifier>>) -> Option<&'static str>;
}

/// Keyboard layout
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Layout {
	/// English keyboard layouts
	En(EnLayout)
}

impl KeyString for Layout {
	fn as_str(&self, code: KeyMap, mods: Option<&HashSet<Modifier>>) -> Option<&'static str> {
		use Layout::*;
		match self {
			En(a) => a.as_str(code, mods)
		}
	}
}

/// English keyboard layouts
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EnLayout {
	Uk,
	Us
}

impl EnLayout {
	fn num(&self, key: KeyMap) -> Option<&'static str> {
		use KeyMap::*;
		Some(match key {
			Numpad0 => "0",
			Numpad1 => "1",
			Numpad2 => "2",
			Numpad3 => "3",
			Numpad4 => "4",
			Numpad5 => "5",
			Numpad6 => "6",
			Numpad7 => "7",
			Numpad8 => "8",
			Numpad9 => "9",
			NumpadDivide => "/",
			NumpadMultiply => "*",
			NumpadSubtract => "-",
			NumpadAdd => "+",
			NumpadDecimal => ".",
			_ => return None
		})
	}

	fn special(&self, key: KeyMap) -> Option<&'static str> {
		use KeyMap::*;
		Some(match key {
			Minus => "-",
			Equal => "=",
			BracketLeft => "[",
			BracketRight => "]",
			Semicolon => ";",
			Quote => "'",
			Backquote => "`",
			Backslash => match self {
				Self::Uk => "#",
				Self::Us => "\\"
			},
			IntlBackslash => match self {
				Self::Uk => "\\",
				Self::Us => "<"
			},
			Comma => ",",
			Period => ".",
			Slash => "/",
			_ => return None
		})
	}

	fn caps(&self, key: KeyMap) -> Option<&'static str> {
		use KeyMap::*;
		Some(match key {
			Digit0 => "0",
			Digit1 => "1",
			Digit2 => "2",
			Digit3 => "3",
			Digit4 => "4",
			Digit5 => "5",
			Digit6 => "6",
			Digit7 => "7",
			Digit8 => "8",
			Digit9 => "9",
			KeyA => "A",
			KeyB => "B",
			KeyC => "C",
			KeyD => "D",
			KeyE => "E",
			KeyF => "F",
			KeyG => "G",
			KeyH => "H",
			KeyI => "I",
			KeyJ => "J",
			KeyK => "K",
			KeyL => "L",
			KeyM => "M",
			KeyN => "N",
			KeyO => "O",
			KeyP => "P",
			KeyQ => "Q",
			KeyR => "R",
			KeyS => "S",
			KeyT => "T",
			KeyU => "U",
			KeyV => "V",
			KeyW => "W",
			KeyX => "X",
			KeyY => "Y",
			KeyZ => "Z",
			_ => return self.special(key)
		})
	}

	fn shift(&self, key: KeyMap) -> Option<&'static str> {
		use KeyMap::*;
		Some(match key {
			Digit0 => ")",
			Digit1 => "!",
			Digit2 => match self {
				Self::Uk => "\"",
				Self::Us => "@"
			},
			Digit3 => match self {
				Self::Uk => "£",
				Self::Us => "#"
			},
			Digit4 => "$",
			Digit5 => "%",
			Digit6 => "^",
			Digit7 => "&",
			Digit8 => "*",
			Digit9 => "(",
			Minus => "_",
			Equal => "+",
			BracketLeft => "{",
			BracketRight => "}",
			Semicolon => ":",
			Quote => match self {
				Self::Uk => "@",
				Self::Us => "\""
			},
			Backquote => match self {
				Self::Uk => "|",
				Self::Us => "~",
			},
			Backslash => match self {
				Self::Uk => "~",
				Self::Us => "|"
			},
			IntlBackslash => match self {
				Self::Uk => "|",
				Self::Us => ">"
			},
			Comma => "<",
			Period => ">",
			Slash => "?",
			_ => return None
		})
	}

	fn none(&self, key: KeyMap) -> Option<&'static str> {
		use KeyMap::*;
		Some(match key {
			Digit0 => "0",
			Digit1 => "1",
			Digit2 => "2",
			Digit3 => "3",
			Digit4 => "4",
			Digit5 => "5",
			Digit6 => "6",
			Digit7 => "7",
			Digit8 => "8",
			Digit9 => "9",
			KeyA => "a",
			KeyB => "b",
			KeyC => "c",
			KeyD => "d",
			KeyE => "e",
			KeyF => "f",
			KeyG => "g",
			KeyH => "h",
			KeyI => "i",
			KeyJ => "j",
			KeyK => "k",
			KeyL => "l",
			KeyM => "m",
			KeyN => "n",
			KeyO => "o",
			KeyP => "p",
			KeyQ => "q",
			KeyR => "r",
			KeyS => "s",
			KeyT => "t",
			KeyU => "u",
			KeyV => "v",
			KeyW => "w",
			KeyX => "x",
			KeyY => "y",
			KeyZ => "z",
			_ => return self.special(key)
		})
	}
}

impl KeyString for EnLayout {
	#[inline]
	fn as_str(&self, key: KeyMap, mods: Option<&HashSet<Modifier>>) -> Option<&'static str> {
		use Modifier::*;
		if let Some(mods) = mods {
			if mods.contains(&Caps) {
				if mods.contains(&Shift) {
					let s = self.shift(key).or_else(|| self.none(key));
					if mods.contains(&NumLock) {
						s.or_else(|| self.num(key))
					} else {
						s
					}
				} else {
					if mods.contains(&NumLock) {
						self.caps(key).or_else(|| self.num(key))
					} else {
						self.caps(key)
					}
				}
			} else if mods.contains(&Shift) {
				let s = self.shift(key).or_else(|| self.caps(key));
				if mods.contains(&NumLock) {
					s.or_else(|| self.num(key))
				} else {
					s
				}
			} else {
				if mods.contains(&NumLock) {
					self.none(key).or_else(|| self.num(key))
				} else {
					self.none(key)
				}
			}
		} else {
			self.none(key)
		}
	}
}

impl super::KeyInput {
	/// Map the keycode to a keyboard character map
	pub fn map(&self) -> Option<KeyMap> {
		(self.map)(self.code)
	}

	/// Converts printable characters into a `&str` using the keyboard
	/// layout `Layout`
	pub fn as_str_using(&self, layout: &Layout) -> Option<&str> {
		use Modifier::*;
		
		let key = self.map()?;
		let mods = self.mods.as_ref().map(|m| m.iter().cloned().collect::<HashSet<_>>());
		layout.as_str(key, mods.as_ref())
	}
}