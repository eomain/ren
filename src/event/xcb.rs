extern crate xcb;

use event::ExposeMap;
use event::KeyMap;
use display::window::Position;

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

fn keymap(code: xcb::ffi::xcb_keycode_t) -> KeyMap
{
    /* TODO */
    KeyMap::SHIFT
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

pub fn mouse_hover(event: &xcb::GenericEvent) -> Position
{
    let pos: &xcb::MotionNotifyEvent = unsafe {
        xcb::cast_event(event)
    };

    (pos.event_x() as u32, pos.event_y() as u32)
}
