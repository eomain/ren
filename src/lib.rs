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
    ManagerName
};
use std::collections::HashMap;

/// A single window session
struct Session {
    batch: HashMap<Token, MessageQueue>,
    context: Context
}

impl Session {
    fn new() -> Self
    {
        let name = ManagerName::default();
        let mut context = Context::new(name);
        context.init();

        Self {
            batch: HashMap::new(),
            context
        }
    }

    fn poll(&self) -> Status
    {
        let event = self.context.event();
        Ok(Message::response(event))
    }

    fn command(&mut self, command: &Command)
    {
        match command {
            Command::Window(command) => {
                self.context.window(command);
            },
            _ => ()
        }
    }

    fn body(&mut self, body: &Body) -> Status
    {
        match body {
            Body::Stat(s) => {
                match self.context.stat(*s) {
                    Some(data) => return Ok(Message::response(data)),
                    _ => ()
                }
            },
            Body::Command(c) => { self.command(c); },
            _ => ()
        }
        Ok(Message::empty())
    }

    fn handle(&mut self, message: &Message) -> Status
    {
        use Type::*;
        match message.ty() {
            Request => self.body(&message.body()),
            _ => Err(Error::Type)
        }
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
    manager: Manager,
    pub event: Option<fn(&Manager) -> Event>,
    pub stat: Option<fn(&Manager, stat: Stat) -> Option<Data>>,
    pub window: Box<Fn(&Manager, &WindowCommand)>
}

impl Context {

    fn new(name: ManagerName) -> Self
    {
        Self {
            name,
            manager: Manager::None,
            event: None,
            stat: None,
            window: Box::new(|_, _| {})
        }
    }
}

impl Context {

    fn init(&mut self)
    {
        self.name().init(self);
        self.manager = Manager::new(&self.name);
    }

    fn name(&self) -> ManagerName
    {
        self.name.clone()
    }

    fn event(&self) -> Event
    {
        match &self.event {
            Some(event) => event(&self.manager),
            _ => Event::None
        }
    }

    fn stat(&self, status: Stat) -> Option<Data>
    {
        match &self.stat {
            Some(stat) => stat(&self.manager, status),
            _ => None
        }
    }

    fn window(&self, command: &WindowCommand)
    {
        (self.window)(&self.manager, command);
    }
}

impl Drop for Context {

    fn drop(&mut self)
    {

    }
}
