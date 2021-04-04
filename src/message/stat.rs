
use super::Body;

/// A type used to get status info
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Stat {
    /// Window status
    Window(WindowStat),
    /// When using XCB
    Xcb(XcbStat)
}

impl From<Stat> for Body {
    fn from(s: Stat) -> Self
    {
        Body::Stat(s)
    }
}

macro_rules! stat_from {
    ($t: ty, $i: ident) => {
        impl From<$t> for Stat {
            fn from(data: $t) -> Self
            {
                Stat::$i(data)
            }
        }
    }
}

/// Window status info
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowStat {
    /// Get the window position
    Position,
    /// Get the window dimensions
    Dimension,
    /// Get the window depth
    Depth
}

stat_from!(WindowStat, Window);
body_from!(WindowStat, Stat);

/// XCB status info
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum XcbStat {
    /// Get the raw connection
    Connection,
    /// Get the window ID
    Window,
    /// Get the Visualtype
    VisualType
}

stat_from!(XcbStat, Xcb);
body_from!(XcbStat, Stat);
