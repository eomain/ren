
use crate::{
    Token,
    Body,
    Message,
    Status,
    Error,
    MessageQueue,
    session::Session
};
use std::collections::HashMap;

/// A `Connection` is used as the channel for communication with the
/// windowing system. Communication is done via `Message`s.
/// You can either a request (Message) and get a response or
/// wait/poll the system for incoming messages such as `Event`s.
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

    /// Begins a new session
    pub fn begin(&mut self) -> Token
    {
        let mut token = Token::new();
        while self.sessions.contains_key(&token) {
            token = Token::new();
        }
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

    /// Send a session request message
    /// # Example
    /// ```
    /// let mut connect = ren::Connection::new();
    /// let token = connect.begin();
    /// connect.request(&token, ren::WindowCommand::Map);
    /// ```
    pub fn request<B>(&mut self, token: &Token, body: B) -> Status
        where B: Into<Body> {
        self.send(token, Message::request(body))
    }

    /// Wait for an `Event` message. This will block until there is a response.
    pub fn wait(&self, token: &Token) -> Status
    {
        match self.sessions.get(token) {
            None => Err(Error::Token),
            Some(session) => session.wait()
        }
    }

    /// Poll for an `Event` message. This is non-blocking.
    /// With the `async-rt` feature enabled,
    /// we can instead use `.session` to get ahold of the
    /// session and call `.await` from an async context
    /// which will resolve to an `Event`.
    /// # Example
    ///```
    /// // Open a connection
    /// let mut connect = ren::Connection::new();
    /// let token = connect.begin();
    ///
    /// // Init code goes here
    ///
    /// // Get the session
    /// let mut session = connect.session(&token).unwrap();
    /// loop {
    ///     // Await the event
    ///     let message = (&mut session).await;
    ///     println!("{:?}", message);
    ///
    ///     match message.body() {
    ///         ren::Body::Event(ren::Event::Terminate) => break,
    ///         _ => ()
    ///     }
    ///}
    ///```

    pub fn poll(&self, token: &Token) -> Status
    {
        match self.sessions.get(token) {
            None => Err(Error::Token),
            Some(session) => session.poll()
        }
    }

    /// Returns a sesssion associated with the connection
    pub fn session(&mut self, token: &Token) -> Option<&mut Session>
    {
        self.sessions.get_mut(token)
    }

    /// Batch a sequence of messages and return a batch token
    pub fn batch(&mut self, token: &Token, mut queue: MessageQueue) -> Result<Token, Error>
    {
        match self.sessions.get_mut(token) {
            None => Err(Error::Token),
            Some(session) => {
                let mut token = Token::new();
                while session.batch.contains_key(&token) {
                    token = Token::new();
                }
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
