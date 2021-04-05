
use crate::{
	Token, Event, Body, Message, Status, Error, MessageQueue,
	session::Session, context::{ConnectionError}, system::{System, SystemType}
};
use std::collections::HashMap;

/// A `Connection` is used as the channel for communication with the
/// windowing system. Communication is done via `Message`s.
/// You can either send a request (Message) and get a response or
/// `.wait/.poll` the system for incoming `Event`s.
/// Using the `async-rt` feature, we can asynchronously await an `Event`
/// using the `.event` method.
pub struct Connection {
	system: System,
    sessions: HashMap<Token, Session>
}

impl Connection {
    /// Open a new connection for communication with the default windowing system
    pub fn open() -> Result<Self, Option<ConnectionError>>
    {
        Self::open_with(SystemType::default())
    }
    
    /// Open a new connection for communication with the windowing system
    pub fn open_with(ty: SystemType) -> Result<Self, Option<ConnectionError>>
    {
        Ok(Self {
        	system: System::new(ty)?,
            sessions: HashMap::new()
        })
    }

    /// Begins a new window session and returns a `Token` as reference
    pub fn begin(&mut self) -> Token
    {
        let mut token = Token::new();
        while self.sessions.contains_key(&token) {
            token = Token::new();
        }
        let window = self.system.create_window();
        let session = Session::new(window);
        self.sessions.insert(token.clone(), session);
        token
    }

    /// End a current window session
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
        // TODO
        self.sessions.contains_key(token)
    }

    /// Send a `Message` to the windowing system
    pub fn send(&mut self, token: &Token, message: Message) -> Status
    {
        match self.sessions.get_mut(token) {
            None => Err(Error::Token),
            Some(session) => session.handle(&message)
        }
    }

    /// Send a request `Message` to the windowing system
    /// # Example
    /// ```
    /// let mut connect = ren::Connection::open().unwrap();
    /// let session = connect.begin();
    /// connect.request(&session, ren::WindowCommand::Map);
    /// ```
    pub fn request<B>(&mut self, token: &Token, body: B) -> Status
        where B: Into<Body> {
        self.send(token, Message::request(body))
    }

    /// Send many request `Message`s to the windowing system at once.
    /// # Example
    /// ```
    /// use ren::WindowCommand::*;
    ///
    /// let mut connect = ren::Connection::open().unwrap();
    /// let session = connect.begin();
    ///
    /// connect.requests(&session, &[
    ///     // Request the window title
    ///     Title(format!("Ren - {}", file!())),
    ///     // Request the window dimensions
    ///     Dimension((640, 480)),
    ///     // Map the window
    ///     Map
    /// ]);
    /// ```
    pub fn requests<T, B>(&mut self, token: &Token, requests: T) -> Status
        where T: AsRef<[B]>, B: Into<Body> + Clone {
            let mut res = Ok(Message::empty());
            for request in requests.as_ref().to_vec() {
                res = self.send(token, Message::request(request));
                if let Err(_) = res {
                    return res;
                }
            }
            res
    }

    /// Wait for an `Event`. This will block until there is a response.
    pub fn wait(&self, token: &Token) -> Result<Event, Error>
    {
        match self.sessions.get(token) {
            None => Err(Error::Token),
            Some(session) => session.wait()
        }
    }

    /// Poll for an `Event`. This is non-blocking.
    pub fn poll(&self, token: &Token) -> Result<Event, Error>
    {
        match self.sessions.get(token) {
            None => Err(Error::Token),
            Some(session) => session.poll()
        }
    }

    /// With the `async-rt` feature enabled,
    /// we can call `.await` from an async context
    /// which will resolve to an `Event`.
    /// # Example
    ///```
    /// use ren::async_std::task;
    ///
    /// task::block_on(async {
    ///     // Open a connection
    ///     let mut connect = ren::Connection::open().unwrap();
    ///     // Create window session
    ///     let session = connect.begin();
    ///
    ///     // Init code goes here
    ///
    ///     loop {
    ///         // Await the event
    ///         let event = connect.event(&session).await.unwrap();
    ///         println!("{:?}", event);
    ///
    ///         match event {
    ///             // Terminate application
    ///             ren::Event::Terminate => break,
    ///             _ => ()
    ///         }
    ///     }
    /// });
    ///```
    #[cfg(feature = "async-rt")]
    pub async fn event(&mut self, token: &Token) -> Result<Event, Error>
    {
        let session = match self.sessions.remove(token) {
            None => return Err(Error::Token),
            Some(session) => session
        };

        let (event, session) = async_std::task::spawn_blocking(move || {
            let event = session.wait();
            (event, session)
        }).await;

        self.sessions.insert(token.clone(), session);
        event.map_err(|_| Error::NoEvent)
    }

    /// Batch a sequence of messages and return a batch token
    pub fn batch(&mut self, token: &Token, queue: MessageQueue) -> Result<Token, Error>
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
