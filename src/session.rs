
use crate::{
    Token,
    Message,
    Command,
    Status,
    Body,
    Error,
    Type,
    MessageQueue,
    context::Context,
    display::{
        Manager,
        ManagerName
    }
};
use std::collections::HashMap;

/// A single window session
pub struct Session {
    pub batch: HashMap<Token, MessageQueue>,
    context: Context
}

impl Session {
    pub fn new() -> Self
    {
        let name = ManagerName::default();
        let mut context = Context::new(name);
        context.init();

        Self {
            batch: HashMap::new(),
            context
        }
    }

    pub fn poll(&self) -> Status
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

    pub fn handle(&mut self, message: &Message) -> Status
    {
        use Type::*;
        match message.ty() {
            Request => self.body(&message.body()),
            _ => Err(Error::Type)
        }
    }

    pub fn run(&mut self, token: &Token) -> Status
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
