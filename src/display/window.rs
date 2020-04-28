
use crate::{
    Context,
    event::Event
};
use super::{
    Manager,
    ManagerName
};
use mirage::surface::Surface;

pub type Position = (u32, u32);

pub type Dimension = (u32, u32);

pub(crate) struct Window {
    init: bool,
    pub title: String,
    pub origin: (u32, u32),
    pub dimension: (u32, u32),
    mapped: bool,
    manager: Manager,
    name: ManagerName
}

impl Window {
    pub fn default(name: ManagerName) -> Self
    {
        Self {
            init: false,
            title: "".into(),
            origin: (0, 0),
            dimension: (0, 0),
            mapped: false,
            manager: Manager::None,
            name
        }
    }

    pub fn map(&mut self, context: &Context)
    {
        if !self.mapped {
            self.manager = Manager::new(&self.name, self);
            if let Some(map) = context.map {
                map(&self.manager);
            }
            self.mapped = true;
        }
    }

    pub fn unmap(&mut self, context: &Context)
    {
        if self.mapped {
            if let Some(unmap) = context.unmap {
                unmap(&self.manager);
            }
            self.mapped = false;
        }
    }

    pub fn draw(&mut self, context: &Context, surface: &Surface)
    {
        if let Some(draw) = context.draw {
            draw(&self.manager, surface);
        }
    }

    pub fn event(&self, context: &Context) -> Event
    {
        if let Some(event) = context.event {
            event(&self.manager)
        } else {
            Event::None
        }
    }

    pub fn title(&self) -> &str
    {
        &self.title
    }

    pub fn origin(&self) -> (u32, u32)
    {
        self.origin
    }

    pub fn dimension(&self) -> (u32, u32)
    {
        self.dimension
    }
}
