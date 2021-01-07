
extern crate xcb;

use super::{Coord, Size, Position, input::{KeyCode, KeyMap}, display::Map};

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
        19 => NUM_0,
        10 => NUM_1,
        11 => NUM_2,
        12 => NUM_3,
        13 => NUM_4,
        14 => NUM_5,
        15 => NUM_6,
        16 => NUM_7,
        17 => NUM_8,
        18 => NUM_9,
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

 pub fn button_press(event: &xcb::GenericEvent) -> Position
 {
     let pos: &xcb::ButtonPressEvent = unsafe {
         xcb::cast_event(event)
     };

     (pos.event_x() as Coord, pos.event_y() as Coord)
 }

 pub fn button_release(event: &xcb::GenericEvent) -> Position
 {
     let pos: &xcb::ButtonReleaseEvent = unsafe {
         xcb::cast_event(event)
     };

     (pos.event_x() as Coord, pos.event_y() as Coord)
 }

pub fn mouse_move(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::MotionNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as Coord, pos.event_y() as Coord)
}

pub fn mouse_enter(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::EnterNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as Coord, pos.event_y() as Coord)
}

pub fn mouse_leave(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::LeaveNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as Coord, pos.event_y() as Coord)
}
