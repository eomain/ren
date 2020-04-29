//! # Message
//! `Message` are the sole form of communication with the library.
//! They consists of a `Type` and `Body`.

extern crate uuid;

use uuid::Uuid;
use mirage::surface::Surface;
use crate::event::Event;

/// The type of the `Message`.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// An empty message
    Empty,
    /// Initialisation message
    Init,
    /// A request
    Request,
    /// A response
    Response,
    /// Custom message type
    Custom(String)
}

/// The body of the message
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    /// Has no body
    None,
    /// A command
    Command(Command),
    /// An event
    Event(Event),
    /// Custom body
    Custom(String)
}

/// A `Message` command. Found within the
/// body of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// A window command
    Window(WindowCommand)
}

impl From<Command> for Body {
    fn from(c: Command) -> Self
    {
        Body::Command(c)
    }
}

impl From<Event> for Body {
    fn from(e: Event) -> Self
    {
        Body::Event(e)
    }
}

/// Commands for window requests.
#[derive(Debug, Clone, PartialEq)]
pub enum WindowCommand {
    /// Request window title
    Title(String),
    /// Request window dimension
    Dimension((u32, u32)),
    /// Request window origin
    Origin((u32, u32)),
    /// Request to map window
    Map,
    /// Request to unmap window
    Unmap,
    /// Request to draw to window
    Draw(Surface)
}

impl From<WindowCommand> for Body {
    fn from(w: WindowCommand) -> Self
    {
        Body::Command(Command::Window(w))
    }
}

/// Error message
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Type,
    Token,
    Custom(String)
}

impl From<&Error> for String {
    fn from(e: &Error) -> Self
    {
        use Error::*;
        match e {
            Type => "specified unexpected message type".into(),
            Token => "specified undefined token".into(),
            Custom(s) => s.into()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", String::from(self))
    }
}

/// The message status
pub type Status = Result<Message, Error>;

/// A message token used to refer to a session
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    uuid: Uuid
}

impl Token {
    pub(crate) fn new() -> Self
    {
        Self {
            uuid: Uuid::new_v4()
        }
    }
}

/// A `Message` for bi-directional communication
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// message type
    ty: Type,
    /// message body
    body: Body
}

impl Message {
    /// Create a new `Message`
    pub fn new<T, B>(ty: T, body: B) -> Self
        where T: Into<Type>, B: Into<Body>
    {
        Self {
            ty: ty.into(),
            body: body.into()
        }
    }

    /// Create a new response `Message`
    pub fn response<B>(body: B) -> Self
        where B: Into<Body>
    {
        Self {
            ty: Type::Response,
            body: body.into()
        }
    }

    /// Create a new request `Message`
    pub fn request<B>(body: B) -> Self
        where B: Into<Body>
    {
        Self {
            ty: Type::Request,
            body: body.into()
        }
    }

    /// Create an empty `Message`
    pub fn empty() -> Self
    {
        Self::new(Type::Empty, Body::None)
    }

    /// Get the type of the message
    pub fn ty(&self) -> &Type
    {
        &self.ty
    }

    /// Get the body of the message
    pub fn body(&self) -> &Body
    {
        &self.body
    }

    /// If the message is empty
    pub fn is_empty(&self) -> bool
    {
        match &self.ty {
            Type::Empty => true,
            _ => false
        }
    }
}

/// The message queue
#[derive(Debug, Clone, PartialEq)]
pub struct MessageQueue {
    messages: Vec<Message>
}

impl MessageQueue {
    /// Create a new message queue
    pub fn new() -> Self
    {
        Self {
            messages: Vec::new()
        }
    }

    /// Append a message to the queue
    pub fn enqueue(&mut self, message: Message)
    {
        self.messages.push(message);
    }

    /// Retrieve the message at the front of the queue
    pub fn front(&mut self) -> Option<Message>
    {
        if self.messages.len() > 0 {
            Some(self.messages.remove(0))
        } else {
            None
        }
    }

    pub(crate) fn join(&mut self, other: &mut Self)
    {
        self.messages.append(&mut other.messages);
    }
}

mod tests {
    use super::*;

    #[test]
    fn message()
    {
        let mut queue = MessageQueue::new();
        queue.enqueue(Message::empty());
        println!("{:?}", queue);
    }

    #[test]
    fn error()
    {
        println!("{}", Error::Token);
    }
}
