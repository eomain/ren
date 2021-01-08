
extern crate xcb;

use std::fmt::{Debug, Formatter, Error};
use super::Body;

/// A type containing status data
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    /// Window data
    Window(WindowData),
    /// When using XCB
    Xcb(XcbData)
}

impl From<Data> for Body {
    fn from(d: Data) -> Self
    {
        Body::Data(d)
    }
}

macro_rules! data_from {
    ($t: ty, $i: ident) => {
        impl From<$t> for Data {
            fn from(data: $t) -> Self
            {
                Data::$i(data)
            }
        }
    }
}

/// Window status data
#[derive(Debug, Clone, PartialEq)]
pub enum WindowData {
    /// Get the window position
    Position((u32, u32)),
    /// Get the window dimensions
    Dimension((u32, u32)),
    /// Get the window depth
    Depth(u8)
}

data_from!(WindowData, Window);
body_from!(WindowData, Data);

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

data_from!(XcbData, Xcb);
body_from!(XcbData, Data);
