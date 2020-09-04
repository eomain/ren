//! # Events
//! In `Ren`, when an event is received
//! an `Event` object is returned.
//! All information about the event,
//! such as the type and relating metadata
//! are contained within this object.
//!
//! ## Events categories:
//!
//! `DisplayEvent` refers to things such as
//! an expose area of the display that needs to
//! updated.
//!
//! `InputEvent` events refer to user input that
//! is generated from devices such as a mouse
//! and keyboard.

pub(crate) mod xcb;

pub type Position = (u32, u32);

pub type Dimension = (u32, u32);

/// All events relating to the display
pub mod display;

/// All events relating to user input
pub mod input;

/// A display event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DisplayEvent {
    /// An area of the display to be updated.
    Expose(display::ExposeMap)
}

impl From<DisplayEvent> for Event {
    fn from(d: DisplayEvent) -> Self
    {
        Event::Display(d)
    }
}

/// An input event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEvent {
    /// A signal for a Key (KeyBoard) event.
    Key(input::KeyEvent),
    /// A signal for a Mouse event.
    Mouse(input::MouseEvent)
}

impl From<InputEvent> for Event {
    fn from(i: InputEvent) -> Self
    {
        Event::Input(i)
    }
}

/// A ren event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    /// No event has occured.
    None,
    /// An event signalling termination of the current display.
    Terminate,
    /// A display type event
    Display(DisplayEvent),
    /// An input event
    Input(InputEvent)
}

impl Event {

    /// if the event is a DisplayEvent
    pub fn is_display(&self) -> bool
    {
        if let Event::Display(_) = self {
            true
        } else {
            false
        }
    }

    /// if the event is an InputEvent
    pub fn is_input(&self) -> bool
    {
        if let Event::Input(_) = self {
            true
        } else {
            false
        }
    }

    /// returns Some, if a KeyEvent otherwise None
    pub fn key_event(&self) -> Option<&input::KeyEvent>
    {
        if let Event::Input(event) = self {
            if let InputEvent::Key(event) = event {
                return Some(event)
            }
        }

        None
    }

}
