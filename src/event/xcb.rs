
extern crate xcb;

use crate::event::display::ExposeMap;
use crate::event::input::KeyMap;
use crate::display::window::Position;

pub fn expose(event: &xcb::GenericEvent) -> ExposeMap
{
    let expose: &xcb::ExposeEvent = unsafe {
        xcb::cast_event(event)
    };

    ExposeMap::new(
        (expose.x() as u32, expose.y() as u32),
        (expose.width() as u32, expose.height() as u32)
    )
}

#[inline]
fn keymap(code: xcb::ffi::xcb_keycode_t) -> KeyMap
{
    /* TODO */
    match code {
        _ => KeyMap::UNKNOWN
    }
}

 pub fn key_press(event: &xcb::GenericEvent) -> KeyMap
 {
     let key: &xcb::KeyPressEvent = unsafe {
         xcb::cast_event(event)
     };

     keymap(key.detail())
 }

 pub fn key_release(event: &xcb::GenericEvent) -> KeyMap
 {
     let key: &xcb::KeyReleaseEvent = unsafe {
         xcb::cast_event(event)
     };

     keymap(key.detail())
 }

 pub fn button_press(event: &xcb::GenericEvent) -> Position
 {
     let pos: &xcb::ButtonPressEvent = unsafe {
         xcb::cast_event(event)
     };

     (pos.event_x() as u32, pos.event_y() as u32)
 }

 pub fn button_release(event: &xcb::GenericEvent) -> Position
 {
     let pos: &xcb::ButtonReleaseEvent = unsafe {
         xcb::cast_event(event)
     };

     (pos.event_x() as u32, pos.event_y() as u32)
 }

pub fn mouse_move(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::MotionNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as u32, pos.event_y() as u32)
}

pub fn mouse_enter(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::EnterNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as u32, pos.event_y() as u32)
}

pub fn mouse_leave(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::LeaveNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as u32, pos.event_y() as u32)
}
