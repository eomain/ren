
use crate::context::{xcb, Context, SystemContext};

pub fn init(context: &mut Context, name: SystemType)
{
    match name {
        SystemType::Xcb =>
            xcb::init(context),
        _ => ()
    }
}

/// The kind of windowing system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemType {
    Xcb
}

impl Default for SystemType {
    #[cfg(target_family = "unix")]
    fn default() -> Self {
        SystemType::Xcb
    }
}

pub enum System {
    Xcb(xcb::Context)
}

impl System {
    pub fn new(sys: SystemType) -> Self
    {
        match sys {
            SystemType::Xcb  => {
                System::Xcb(xcb::Context::init())
            }
        }
    }

    pub fn xcb(&self) -> Option<&xcb::Context>
    {
        match self {
            System::Xcb(context) => Some(context)
        }
    }

    pub fn xcb_unwrap(&self) -> &xcb::Context
    {
        match self {
            System::Xcb(context) => context,
            _ => panic!("error: not an XCB context")
        }
    }
}
