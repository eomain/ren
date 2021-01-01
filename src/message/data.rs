
extern crate xcb;

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
#[derive(Debug, Clone, PartialEq)]
pub enum XcbData {
    /// Get the raw connection
    Connection(*mut xcb::ffi::xcb_connection_t),
    /// Get the window ID
    Window(xcb::Window)
}

unsafe impl Send for XcbData {}
unsafe impl Sync for XcbData {}

impl From<XcbData> for Data {
    fn from(data: XcbData) -> Self
    {
        Data::Xcb(data)
    }
}
