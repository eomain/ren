
pub mod xcb;

use std::sync::Arc;
use crate::{
    Stat,
    Data,
    WindowCommand,
    render::Surface,
    event::Event,
    system::{
        init,
        System,
        SystemType
    }
};

pub struct Context {
    pub ty: SystemType,
    system: System,
    pub event: Box<dyn Fn(&System) -> Option<Event> + Send + Sync>,
    pub poll: Box<dyn Fn(&System) -> Option<Event> + Send + Sync>,
    pub stat: Box<dyn Fn(&System, Stat) -> Option<Data> + Send + Sync>,
    pub window: Box<dyn Fn(&System, &WindowCommand) + Send + Sync>
}

impl Context {
    pub fn new(ty: SystemType) -> Self {
        Self {
            ty,
            system: System::new(ty),
            event: Box::new(|_| None),
            poll: Box::new(|_| None),
            stat: Box::new(|_, _| None),
            window: Box::new(|_, _| {})
        }
    }

    pub fn init(&mut self) {
        init(self, self.ty);
    }

    pub fn event(&self) -> Option<Event> {
        (self.event)(&self.system)
    }

    pub fn poll(&self) -> Option<Event> {
        (self.poll)(&self.system)
    }

    pub fn stat(&self, status: Stat) -> Option<Data> {
        (self.stat)(&self.system, status)
    }

    pub fn window(&self, command: &WindowCommand) {
        (self.window)(&self.system, command);
    }
}

pub trait SystemContext {

    fn init() -> Self;

    fn event(&self) -> Option<Event>;

    fn poll(&self) -> Option<Event>;

    fn stat(&self, _: Stat) -> Option<Data>;

    fn window(&self, _: &WindowCommand);

    fn update(&self);
}
