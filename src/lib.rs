//! # Ren

//! `Ren` is library that provides basic access
//! to the client windowing system. There is
//! support for rendering primative drawing operations to
//! the window surface. All communication is done via messages.

extern crate mirage;

mod context;
mod display;
mod session;
mod connection;

pub mod event;
pub mod render;
pub mod message;

mod prelude;
pub use prelude::*;
pub use connection::Connection;
