
use crate::{
    Token,
    Message,
    Status,
    Error,
    MessageQueue,
    session::Session
};
use std::collections::HashMap;

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
