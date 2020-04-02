
pub mod window;

use crate::Context;
use crate::context::DisplayContext;
use crate::context::xcb;
use self::window::Window;

#[derive(Clone)]
pub enum ManagerName {
    None,
    XCB
}

impl ManagerName {

    pub fn default() -> Self
    {
        ManagerName::XCB
    }

}

impl ManagerName {

    pub fn init(&self, context: &mut Context)
    {
        match self {
            ManagerName::XCB =>
                xcb::init(context),

            _ => ()
        }
    }

}

pub enum Manager {
    None,
    XCB(xcb::Context)
}

impl Manager {

    pub fn init(name: &ManagerName, window: &mut Window)
    {
        match *name {
            ManagerName::None => (),
            ManagerName::XCB  => {
                let manager = Manager::XCB(xcb::Context::init(window));
                window.set_manager(manager);
            }
        }
    }
}

impl Manager {

    pub fn xcb(&self) -> &xcb::Context
    {
        match self {
            Manager::XCB(context) => context,
            _ => panic!()
        }
    }

}
