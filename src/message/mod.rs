//! # Message
//! `Message` are the sole form of communication with the library.
//! They consists of a `Type` and `Body`.

extern crate uuid;

pub mod data;
pub mod stat;

use stat::Stat;
use data::Data;
use uuid::Uuid;
use crate::render::{
    Image,
    Surface
};
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
    /// Status
    Stat(Stat),
    /// Status data
    Data(Data),
    /// A command
    Command(Command),
    /// An event
    Event(Event),
    /// Custom body
    Custom(String)
}

impl Default for Body {
    fn default() -> Self
    {
        Body::None
    }
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
    /// Request to stack the window above
    StackAbove,
    /// Request to stack the window below
    StackBelow,
    /// Request to draw to window
    Draw(Surface),
    /// Request to draw image to window
    Image(Image),
    /// Request to update the window
    Update
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

    /// Take the body of the message
    pub fn take_body(&mut self) -> Body
    {
        std::mem::take(&mut self.body)
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
    messages: Vec<Message>,
    limit: Option<usize>
}

impl MessageQueue {
    /// Create a new message queue
    pub fn new() -> Self
    {
        Self {
            messages: Vec::new(),
            limit: None
        }
    }

    /// Append a message to the queue
    pub fn enqueue(&mut self, message: Message)
    {
        if let Some(0) = self.limit {
            return;
        }

        if self.full() {
            self.front();
        }
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

    /// Optionally add a limit on the queue size
    pub fn limit(&mut self, limit: Option<usize>)
    {
        self.limit = limit;
    }

    /// Get the number of messages in the queue
    #[inline]
    pub fn size(&self) -> usize
    {
        self.messages.len()
    }

    /// Check if the queue is full
    #[inline]
    pub fn full(&self) -> bool
    {
        match self.limit {
            None => false,
            Some(limit) => if limit == self.size() {
                true
            } else {
                false
            }
        }
    }

    pub(crate) fn messages(&self) -> &[Message]
    {
        &self.messages
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
        queue.limit(Some(0));
        queue.enqueue(Message::empty());
        println!("{:?}", queue);
        assert_eq!(true, queue.full());
    }

    #[test]
    fn error()
    {
        println!("{}", Error::Token);
    }
}
