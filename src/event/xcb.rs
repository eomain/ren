
extern crate xcb;

use super::{Coord, Size, Position, input::{KeyInput, KeyMap, KeyCode, MouseInput, keyboard}, display::Map};

pub fn expose(event: &xcb::GenericEvent) -> Map
{
	let expose: &xcb::ExposeEvent = unsafe {
		xcb::cast_event(event)
	};

	Map (
		(expose.x() as Coord, expose.y() as Coord),
		(expose.width() as Size, expose.height() as Size)
	)
}

fn modifier(state: u16) -> Option<keyboard::Modifiers> {
	use keyboard::Modifier::*;
	Some(match state {
		0x01 => &[Shift],
		0x02 => &[Caps],
		0x03 => &[Shift, Caps],
		0x04 => &[Ctrl],
		0x05 => &[Ctrl, Shift],
		0x06 => &[Ctrl, Caps],
		0x07 => &[Ctrl, Shift, Caps],
		0x08 => &[Alt],
		0x09 => &[Alt, Shift],
		0x0A => &[Alt, Caps],
		0x0B => &[Alt, Shift, Caps],
		0x0C => &[Ctrl, Alt],
		0x0D => &[Ctrl, Alt, Shift],
		0x0E => &[Ctrl, Alt, Caps],
		0x0F => &[Ctrl, Alt, Shift, Caps],
		0x10 => &[NumLock],
		0x11 => &[NumLock, Shift],
		0x12 => &[NumLock, Caps],
		0x13 => &[NumLock, Caps, Shift],
		0x14 => &[NumLock, Ctrl],
		0x15 => &[NumLock, Ctrl, Shift],
		0x16 => &[NumLock, Ctrl, Caps],
		0x17 => &[NumLock, Ctrl, Shift, Caps],
		0x18 => &[NumLock, Alt],
		0x19 => &[NumLock, Alt, Shift],
		0x1A => &[NumLock, Alt, Caps],
		0x1B => &[NumLock, Alt, Shift, Caps],
		0x1C => &[NumLock, Ctrl, Alt],
		0x1D => &[NumLock, Ctrl, Alt, Shift],
		0x1E => &[NumLock, Ctrl, Alt, Caps],
		0x1F => &[NumLock, Ctrl, Alt, Shift, Caps],
		_ => return None
	})
}

#[allow(dead_code)]
fn keymap(code: KeyCode) -> Option<KeyMap> {
	use KeyMap::*;
	Some(match code {
		0x09 => Escape,
		0x0A => Digit1,
		0x0B => Digit2,
		0x0C => Digit3,
		0x0D => Digit4,
		0x0E => Digit5,
		0x0F => Digit6,
		0x10 => Digit7,
		0x11 => Digit8,
		0x12 => Digit9,
		0x13 => Digit0,
		0x14 => Minus,
		0x15 => Equal,
		0x16 => Backspace,
		0x17 => Tab,
		0x18 => KeyQ,
		0x19 => KeyW,
		0x1A => KeyE,
		0x1B => KeyR,
		0x1C => KeyT,
		0x1D => KeyY,
		0x1E => KeyU,
		0x1F => KeyI,
		0x20 => KeyO,
		0x21 => KeyP,
		0x22 => BracketLeft,
		0x23 => BracketRight,
		0x24 => Enter,
		0x25 => CtrlLeft,
		0x26 => KeyA,
		0x27 => KeyS,
		0x28 => KeyD,
		0x29 => KeyF,
		0x2A => KeyG,
		0x2B => KeyH,
		0x2C => KeyJ,
		0x2D => KeyK,
		0x2E => KeyL,
		0x2F => Semicolon,
		0x30 => Quote,
		0x31 => Backquote,
		0x32 => ShiftLeft,
		0x33 => Backslash,
		0x34 => KeyZ,
		0x35 => KeyX,
		0x36 => KeyC,
		0x37 => KeyV,
		0x38 => KeyB,
		0x39 => KeyN,
		0x3A => KeyM,
		0x3B => Comma,
		0x3C => Period,
		0x3D => Slash,
		0x3E => ShiftRight,
		0x3F => NumpadMultiply,
		0x40 => AltLeft,
		0x41 => Space,
		0x42 => CapsLock,
		0x43 => F1,
		0x44 => F2,
		0x45 => F3,
		0x46 => F4,
		0x47 => F5,
		0x48 => F6,
		0x49 => F7,
		0x4A => F8,
		0x4B => F9,
		0x4C => F10,
		0x4F => Numpad7,
		0x50 => Numpad8,
		0x51 => Numpad9,
		0x52 => NumpadSubtract,
		0x53 => Numpad4,
		0x54 => Numpad5,
		0x55 => Numpad6,
		0x56 => NumpadAdd,
		0x57 => Numpad1,
		0x58 => Numpad2,
		0x59 => Numpad3,
		0x5A => Numpad0,
		0x5B => NumpadDecimal,
		0x5E => IntlBackslash,
		0x5F => F11,
		0x60 => F12,
		0x69 => CtrlRight,
		0x6A => NumpadDivide,
		0x6B => PrintScreen,
		0x6C => AltRight,
		0x6E => Home,
		0x6F => ArrowUp,
		0x71 => ArrowLeft,
		0x72 => ArrowRight,
		0x73 => End,
		0x74 => ArrowDown,
		0x75 => PageDown,
		0x76 => Insert,
		0x77 => Delete,
		0x79 => AudioVolumeMute,
		0x7A => AudioVolumeDown,
		0x7B => AudioVolumeUp,
		0x7F => Pause,
		0x87 => ContextMenu,
		_ => return None
	})
}

pub fn key_press(event: &xcb::GenericEvent) -> KeyInput
{
	let key: &xcb::KeyPressEvent = unsafe {
		xcb::cast_event(event)
	};
	let mods = modifier(key.state());
	KeyInput::new(key.detail() as KeyCode, mods, keymap)
}

pub fn key_release(event: &xcb::GenericEvent) -> KeyInput
{
	let key: &xcb::KeyReleaseEvent = unsafe {
		xcb::cast_event(event)
	};

	let mods = modifier(key.state());
	KeyInput::new(key.detail() as KeyCode, mods, keymap)
}

fn button(code: xcb::Button) -> MouseInput {
	match code {
		1 => MouseInput::Left,
		2 => MouseInput::Middle,
		3 => MouseInput::Right,
		4 => MouseInput::ScrollUp,
		5 => MouseInput::ScrollDown,
		_ => MouseInput::Unknown(Some(code))
	}
}

pub fn button_press(event: &xcb::GenericEvent) -> (Position, MouseInput)
{
	let e: &xcb::ButtonPressEvent = unsafe {
		xcb::cast_event(event)
	};

	((e.event_x() as Coord, e.event_y() as Coord), button(e.detail()))
}

pub fn button_release(event: &xcb::GenericEvent) -> (Position, MouseInput)
{
	let e: &xcb::ButtonReleaseEvent = unsafe {
		xcb::cast_event(event)
	};

	((e.event_x() as Coord, e.event_y() as Coord), button(e.detail()))
}

pub fn mouse_move(event: &xcb::GenericEvent) -> Position
{
	let e: &xcb::MotionNotifyEvent = unsafe {
		xcb::cast_event(event)
	};

	(e.event_x() as Coord, e.event_y() as Coord)
}

pub fn mouse_enter(event: &xcb::GenericEvent) -> Position
{
	let e: &xcb::EnterNotifyEvent = unsafe {
		xcb::cast_event(event)
	};

	(e.event_x() as Coord, e.event_y() as Coord)
}

pub fn mouse_leave(event: &xcb::GenericEvent) -> Position
{
	let e: &xcb::LeaveNotifyEvent = unsafe {
		xcb::cast_event(event)
	};

	(e.event_x() as Coord, e.event_y() as Coord)
}
