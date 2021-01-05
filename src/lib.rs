//! `Ren` is library that provides basic access
//! to the client windowing system. There is
//! support for rendering primative drawing operations to
//! the window surface. All communication is done via messages.
//!
//! # Basic example
//! Display a small window on screen
//! ```
//! use ren::WindowCommand::*;
//!
//! // Open a connection
//! let mut connect = ren::Connection::open().unwrap();
//!
//! // Create window session
//! let session = connect.begin();
//!
//! // Request the window title
//! connect.request(&session, Title(format!("Ren - example")));
//!
//! // Request the window dimensions
//! connect.request(&session, Dimension((320, 240)));
//!
//! // Map the window
//! connect.request(&session, Map);
//!
//! // Update the window
//! connect.request(&session, Update);
//!
//! loop {
//!     // Wait for an event
//!     let event = connect.wait(&session).unwrap();
//!     println!("{:?}", event);
//!
//!     match event {
//!         // Terminate application
//!         ren::Event::Terminate => break,
//!         _ => ()
//!     }
//! }
//! ```

extern crate mirage;

mod context;
mod system;
mod session;
mod connection;

pub mod event;
pub mod graphics;
pub mod render;
pub mod message;

mod prelude;
pub use prelude::*;
pub use connection::Connection;

#[cfg(feature = "async-rt")]
pub use async_std;
