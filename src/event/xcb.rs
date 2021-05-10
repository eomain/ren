
extern crate xcb;

use super::{Coord, Size, Position, input::{KeyCode, KeyMap}, display::Map, input::MouseInput};

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

#[inline]
#[allow(dead_code)]
fn keymap(code: xcb::ffi::xcb_keycode_t) -> KeyMap
{
	/* TODO */
	use KeyMap::*;
	match code {
		9 => Esc,
		67 => F1,
		68 => F2,
		69 => F3,
		70 => F4,
		71 => F5,
		72 => F6,
		73 => F7,
		74 => F8,
		75 => F9,
		76 => F10,
		95 => F11,
		96 => F12,
		19 => Num0,
		10 => Num1,
		11 => Num2,
		12 => Num3,
		13 => Num4,
		14 => Num5,
		15 => Num6,
		16 => Num7,
		17 => Num8,
		18 => Num9,
		50 | 62 => Shift,
		66 => Caps,
		111 => Up,
		116 => Down,
		113 => Left,
		114 => Right,
		_ => Unknown(code as u16)
	}
}

pub fn key_press(event: &xcb::GenericEvent) -> KeyCode
{
	let key: &xcb::KeyPressEvent = unsafe {
		xcb::cast_event(event)
	};

	KeyCode::new(key.detail() as u16)
}

pub fn key_release(event: &xcb::GenericEvent) -> KeyCode
{
	let key: &xcb::KeyReleaseEvent = unsafe {
		xcb::cast_event(event)
	};

	KeyCode::new(key.detail() as u16)
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
