
use super::Body;

/// A type used to get status info
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Stat {
    /// When using XCB
    Xcb(XcbStat)
}

impl From<Stat> for Body {
    fn from(s: Stat) -> Self
    {
        Body::Stat(s)
    }
}

/// XCB status info
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum XcbStat {
    /// Get the raw connection
    Connection,
    /// Get the window ID
    Window,
    /// Get the Visualtype
    VisualType
}

impl From<XcbStat> for Stat {
    fn from(stat: XcbStat) -> Self
    {
        Stat::Xcb(stat)
    }
}

impl From<XcbStat> for Body {
    fn from(s: XcbStat) -> Self
    {
        Body::Stat(s.into())
    }
}
