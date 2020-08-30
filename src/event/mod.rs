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
pub mod display {

    use super::Position;
    use super::Dimension;

    /// A map of an area of the display
    /// that needs to be updated.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct ExposeMap {
        /// The position within the display.
        pos: Position,
        /// The dimensions of the map.
        dim: Dimension
    }

    impl ExposeMap {

        pub fn new(pos: Position, dim: Dimension) -> Self
        {
            Self {
                pos,
                dim
            }
        }
    }

    impl ExposeMap {

        pub fn position(&self) -> Position
        {
            self.pos
        }

        pub fn dimension(&self) -> Dimension
        {
            self.dim
        }
    }

}

/// All events relating to user input
pub mod input {

    use super::Position;


    /// A mapping of KeyBoard events.
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

        NUM_0,
        NUM_1,
        NUM_2,
        NUM_3,
        NUM_4,
        NUM_5,
        NUM_6,
        NUM_7,
        NUM_8,
        NUM_9,

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
        Unknown
    }

    /// The type of Key event.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum KeyEvent {
        /// A Key press has occured.
        Press(KeyMap),
        /// A Key release has occured.
        Release(KeyMap)
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct ButtonMap {
        pos: Position
    }

    impl ButtonMap {

        pub fn position(&self) -> Position
        {
            self.pos
        }
    }

    /// The type of mouse event
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum MouseEvent {
        /// A Mouse press has occured.
        Press(Position),
        /// A Mouse release has occured.
        Release(Position),
        /// A Mouse movement has occured.
        Move(Position),
        /// The Mouse pointer has entered the Window
        Enter(Position),
        /// The Mouse pointer has left the Window
        Leave(Position)
    }

}

/// A display event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DisplayEvent {
    /// An area of the display to be updated.
    Expose(display::ExposeMap)
}

/// An input event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEvent {
    /// A signal for a Key (KeyBoard) event.
    Key(input::KeyEvent),
    /// A signal for a Mouse event.
    Mouse(input::MouseEvent)
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
