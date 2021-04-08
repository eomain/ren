
extern crate xcb;

use std::{fmt::{Debug, Formatter, Error}, sync::Arc};
use super::Body;

/// A type containing status data
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Clone)]
pub enum XcbData {
    /// Get the connection
    Connection(Arc<xcb::Connection>),
    /// Get the window ID
    Window(xcb::Window),
    /// Get the visualtype
    VisualType(xcb::Visualtype),
    /// Pixmap
    Pixmap(xcb::Pixmap)
}

impl PartialEq for XcbData {
    fn eq(&self, other: &Self) -> bool {
        use XcbData::*;
        match (self, other) {
            (Connection(a), Connection(b)) => a.get_raw_conn() == b.get_raw_conn(),
            (Window(a), Window(b)) => a == b,
            (Pixmap(a), Pixmap(b)) => a == b,
            _ => false
        }
    }
}

impl Debug for XcbData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use XcbData::*;
        match self {
            Connection(c) => c.get_raw_conn().fmt(f),
            Window(w) => w.fmt(f),
            VisualType(_) => write!(f, "Visualtype"),
            Pixmap(p) => p.fmt(f)
        }
    }
}

unsafe impl Send for XcbData {}
unsafe impl Sync for XcbData {}

data_from!(XcbData, Xcb);
body_from!(XcbData, Data);
