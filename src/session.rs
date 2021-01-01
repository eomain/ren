
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
    system::SystemType
};
use std::pin::Pin;
use std::task::Poll;
use std::future::Future;
use std::collections::HashMap;

/// A single window session
pub struct Session {
    pub batch: HashMap<Token, MessageQueue>,
    context: Context
}

impl Session {
    pub fn new() -> Self
    {
        let sys = SystemType::default();
        let mut context = Context::new(sys);
        context.init();

        Self {
            batch: HashMap::new(),
            context
        }
    }

    pub fn wait(&self) -> Status
    {
        let event = self.context.event();
        event.map(|e| Message::response(e)).ok_or(Error::NoEvent)
    }

    pub fn poll(&self) -> Status
    {
        let event = self.context.poll();
        event.map(|e| Message::response(e)).ok_or(Error::NoEvent)
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

macro_rules! session_fut {
    ($t:ty) => {
        impl Future for $t {
            type Output = Message;

            fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
                use crate::{Body, Event};
                match Session::poll(&self) {
                    Err(e) => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    },
                    Ok(event) => Poll::Ready(event)
                }
            }
        }
    }
}

#[cfg(feature = "async-rt")]
session_fut!(Session);
#[cfg(feature = "async-rt")]
session_fut!(&Session);
#[cfg(feature = "async-rt")]
session_fut!(&&Session);
