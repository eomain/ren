
extern crate xcb;

use std::fmt::{Debug, Formatter, Error};
use super::Body;

/// A type containing status data
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    /// When using XCB
    Xcb(XcbData)
}

impl From<Data> for Body {
    fn from(d: Data) -> Self
    {
        Body::Data(d)
    }
}

/// XCB status data
#[derive(Clone)]
pub enum XcbData {
    /// Get the raw connection
    Connection(*mut xcb::ffi::xcb_connection_t),
    /// Get the window ID
    Window(xcb::Window),
    /// Get the visualtype
    VisualType(Option<xcb::Visualtype>)
}

impl PartialEq for XcbData {
    fn eq(&self, other: &Self) -> bool {
        use XcbData::*;
        match (self, other) {
            (Connection(a), Connection(b)) => a == b,
            (Window(a), Window(b)) => a == b,
            _ => false
        }
    }
}

impl Debug for XcbData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use XcbData::*;
        match self {
            (Connection(c)) => c.fmt(f),
            (Window(w)) => w.fmt(f),
            (VisualType(_)) => write!(f, "Visualtype")
        }
    }
}

unsafe impl Send for XcbData {}
unsafe impl Sync for XcbData {}

impl From<XcbData> for Data {
    fn from(data: XcbData) -> Self
    {
        Data::Xcb(data)
    }
}
