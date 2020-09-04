
use crate::{
    context::{
        Context,
        DisplayContext,
        xcb
    }
};

pub fn init(context: &mut Context, name: ManagerName)
{
    match name {
        ManagerName::Xcb =>
            xcb::init(context),
        _ => ()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ManagerName {
    None,
    Xcb
}

impl ManagerName {
    pub fn default() -> Self
    {
        // TODO
        ManagerName::Xcb
    }
}

pub enum Manager {
    None,
    Xcb(xcb::Context)
}

impl Manager {
    pub fn new(name: ManagerName) -> Manager
    {
        match name {
            ManagerName::None => Manager::None,
            ManagerName::Xcb  => {
                Manager::Xcb(xcb::Context::init())
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

    pub fn xcb_unwrap(&self) -> &xcb::Context
    {
        match self {
            Manager::Xcb(context) => context,
            _ => panic!("error: not an XCB context")
        }
    }
}
