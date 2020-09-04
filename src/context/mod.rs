
pub mod xcb;

use crate::{
    Stat,
    Data,
    WindowCommand,
    render::Surface,
    event::Event,
    display::{
        init,
        Manager,
        ManagerName
    }
};

pub struct Context {
    pub name: ManagerName,
    manager: Manager,
    pub event: Box<Fn(&Manager) -> Event>,
    pub stat: Box<Fn(&Manager, Stat) -> Option<Data>>,
    pub window: Box<Fn(&Manager, &WindowCommand)>
}

impl Context {

    pub fn new(name: ManagerName) -> Self
    {
        Self {
            name,
            manager: Manager::new(name),
            event: Box::new(|_| Event::None),
            stat: Box::new(|_, _| None),
            window: Box::new(|_, _| {})
        }
    }

}

impl Context {

    pub fn init(&mut self)
    {
        init(self, self.name);
    }

    pub fn event(&self) -> Event
    {
        (self.event)(&self.manager)
    }

    pub fn stat(&self, status: Stat) -> Option<Data>
    {
        (self.stat)(&self.manager, status)
    }

    pub fn window(&self, command: &WindowCommand)
    {
        (self.window)(&self.manager, command);
    }
}

pub(crate) trait DisplayContext {

    fn init() -> Self;

    fn event(&self) -> Event;

    fn stat(&self, _: Stat) -> Option<Data>;

    fn window(&self, _: &WindowCommand);

    fn update(&self);
}
