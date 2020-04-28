
pub mod window;

pub(crate) use window::Window;

use crate::{
    Context,
    context::{
        DisplayContext,
        xcb
    }
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ManagerName {
    None,
    Xcb
}

impl ManagerName {
    pub fn default() -> Self
    {
        // TODO
        ManagerName::Xcb
    }

    pub fn init(&self, context: &mut Context)
    {
        match self {
            ManagerName::Xcb =>
                xcb::init(context),

            _ => ()
        }
    }
}

pub(crate) enum Manager {
    None,
    Xcb(xcb::Context)
}

impl Manager {
    pub fn new(name: &ManagerName, window: &crate::Window) -> Manager
    {
        match *name {
            ManagerName::None => Manager::None,
            ManagerName::Xcb  => {
                Manager::Xcb(xcb::Context::init(window))
            }
        }
    }
}

impl Manager {
    pub fn xcb(&self) -> Option<&xcb::Context>
    {
        match self {
            Manager::Xcb(context) => Some(context),
            _ => None
        }
    }
}
