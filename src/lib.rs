//! # Ren

//! `Ren` is library that provides basic access
//! to the client windowing system. There is
//! support for rendering primative drawing operations to
//! the window surface. All communication is done via messages.

extern crate mirage;

mod context;
mod display;

pub mod event;
pub mod render;
pub mod message;

mod prelude;
pub use prelude::*;

use render::Surface;
use display::{
    Manager,
    ManagerName,
    Window
};
use std::collections::HashMap;

/// A single window session
struct Session {
    batch: HashMap<Token, MessageQueue>,
    context: Context,
    window: Window
}

impl Session {
    fn new() -> Self
    {
        let name = ManagerName::default();
        let mut context = Context::new(name);
        context.init();

        Self {
            batch: HashMap::new(),
            context,
            window: Window::default(name)
        }
    }

    fn poll(&self) -> Status
    {
        let event = self.window.event(&self.context);
        Ok(Message::response(event))
    }

    fn command(&mut self, command: &Command)
    {
        match command {
            Command::Window(w) => {
                use WindowCommand::*;
                match w {
                    Title(title) => {
                        self.window.title = title.into();
                    },
                    Dimension(dimension) => {
                        self.window.dimension = *dimension;
                    },
                    Origin(origin) => {
                        self.window.origin = *origin;
                    },
                    Map => self.window.map(&self.context),
                    Unmap => self.window.unmap(&self.context),
                    Draw(s) => self.window.draw(&self.context, &s)
                }
            },
            _ => ()
        }
    }

    fn body(&mut self, body: &Body)
    {
        match body {
            Body::Command(c) => self.command(c),
            _ => ()
        }
    }

    fn handle(&mut self, message: &Message) -> Status
    {
        use Type::*;
        match message.ty() {
            Request => self.body(&message.body),
            _ => return Err(Error::Type)
        }
        Ok(Message::empty())
    }

    fn run(&mut self, token: &Token) -> Status
    {
        let queue = match self.batch.remove(token) {
            None => return Err(Error::Token),
            Some(queue) => queue
        };

        for message in queue.messages() {
            self.handle(message);
        }

        self.batch.insert(token.clone(), queue);

        Ok(Message::empty())
    }
}

/// A `Connection` is used as the channel for communication
pub struct Connection {
    sessions: HashMap<Token, Session>
}

impl Connection {
    /// Create a new connection for communication
    pub fn new() -> Self
    {
        Self {
            sessions: HashMap::new()
        }
    }

    /// Begin a new session
    pub fn begin(&mut self) -> Token
    {
        let token = Token::new();
        let session = Session::new();
        self.sessions.insert(token.clone(), session);
        token
    }

    /// End a current session
    pub fn end(&mut self, token: &Token) -> Status
    {
        match self.sessions.remove(token) {
            None => Err(Error::Token),
            Some(_) => Ok(Message::empty())
        }
    }

    /// Check if the connection is active
    pub fn active(&self, token: &Token) -> bool
    {
        self.sessions.contains_key(token)
    }

    /// Send a session message
    pub fn send(&mut self, token: &Token, message: Message) -> Status
    {
        match self.sessions.get_mut(token) {
            None => Err(Error::Token),
            Some(session) => session.handle(&message)
        }
    }

    /// Wait for an event message
    pub fn wait(&self, token: &Token) -> Status
    {
        match self.sessions.get(token) {
            None => Err(Error::Token),
            Some(session) => session.poll()
        }
    }

    /// Batch a sequence of messages and return a batch token
    pub fn batch(&mut self, token: &Token, mut queue: MessageQueue) -> Result<Token, Error>
    {
        match self.sessions.get_mut(token) {
            None => Err(Error::Token),
            Some(session) => {
                let token = Token::new();
                session.batch.insert(token.clone(), queue);
                Ok(token)
            }
        }
    }

    /// Dispatch the message queue using a batch token
    pub fn dispatch(&mut self, token: &Token, batch: &Token) -> Status
    {
        match self.sessions.get_mut(token) {
            None => Err(Error::Token),
            Some(session) => session.run(batch)
        }
    }
}

pub(crate) struct Context {
    name: ManagerName,
    pub map: Option<fn(&Manager)>,
    pub unmap: Option<fn(&Manager)>,
    pub draw: Option<fn(&Manager, &render::Surface)>,
    pub event: Option<fn(&Manager) -> Event>
}

impl Context {

    fn new(name: ManagerName) -> Self
    {
        Self {
            name,
            map: None,
            unmap: None,
            draw: None,
            event: None
        }
    }
}

impl Context {

    fn init(&mut self)
    {
        self.name().init(self);
    }

    fn name(&self) -> ManagerName
    {
        self.name.clone()
    }
}

impl Drop for Context {

    fn drop(&mut self)
    {

    }
}
