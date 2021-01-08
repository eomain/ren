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

/// All events relating to the display
pub mod display;

/// All events relating to user input
pub mod input;

pub(crate) mod xcb;

pub type Coord = i16;
pub type Size = u16;

pub type Position = (Coord, Coord);

pub type Dimension = (Size, Size);

macro_rules! event_from {
    ($t: ty, $i: ident) => {
        impl From<$t> for Event {
            fn from(e: $t) -> Self
            {
                Event::$i(e)
            }
        }
    }
}

/// A display event
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DisplayEvent {
    /// An area of the window that needs to be updated.
    Expose(display::Map),
    /// The window gained focus
    FocusIn,
    /// The window lost focus
    FocusOut
}

event_from!(DisplayEvent, Display);

/// An input event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEvent {
    /// A signal for a Key (KeyBoard) event.
    Key(input::KeyEvent),
    /// A signal for a Mouse event.
    Mouse(input::MouseEvent)
}

event_from!(InputEvent, Input);

/// An `Event`
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    /// An unknown event occurred. May contain an event code
    Unknown(Option<u16>),
    /// An event signalling termination of the current application window.
    Terminate,
    /// A display type event
    Display(DisplayEvent),
    /// An input event from a user
    Input(InputEvent)
}

impl Event {

    /// If the event is a `DisplayEvent`
    pub fn is_display(&self) -> bool
    {
        if let Event::Display(_) = self {
            true
        } else {
            false
        }
    }

    /// If the event is an `InputEvent`
    pub fn is_input(&self) -> bool
    {
        if let Event::Input(_) = self {
            true
        } else {
            false
        }
    }

    /// Returns `Some` if a `KeyEvent` otherwise `None`
    pub fn key(&self) -> Option<&input::KeyEvent>
    {
        match self {
            Event::Input(InputEvent::Key(event)) => Some(event),
            _ => None
        }
    }

    /// Returns `Some` if a `MouseEvent` otherwise `None`
    pub fn mouse(&self) -> Option<&input::MouseEvent>
    {
        match self {
            Event::Input(InputEvent::Mouse(event)) => Some(event),
            _ => None
        }
    }
}
