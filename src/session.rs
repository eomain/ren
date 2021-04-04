
use crate::{
    Token, Event, Message, Command, Status, Body, Error, Type,
    MessageQueue, system::Window
};
use std::collections::HashMap;

/// A single window session
pub struct Session {
	window: Window,
    pub batch: HashMap<Token, MessageQueue>
}

impl Session {
    pub fn new(window: Window) -> Self
    {
        Self {
        	window,
            batch: HashMap::new()
        }
    }

    pub fn wait(&self) -> Result<Event, Error>
    {
        let event = self.window.event();
        event.ok_or(Error::NoEvent)
    }

    pub fn poll(&self) -> Result<Event, Error>
    {
        let event = self.window.poll();
        event.ok_or(Error::NoEvent)
    }

    fn command(&mut self, command: &Command)
    {
        match command {
            Command::Window(command) => {
                self.window.window(command);
            },
            _ => ()
        }
    }

    fn body(&mut self, body: &Body) -> Status
    {
        match body {
            Body::Stat(s) => {
                match self.window.stat(*s) {
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
