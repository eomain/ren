
use Context;
use super::Manager;

pub type Position = (u32, u32);

pub type Dimension = (u32, u32);

pub struct Window<'a> {
    title: String,
    origin: Position,
    dimension: Dimension,
    mapped: bool,
    manager: Manager,
    context: &'a Context
}

impl<'a> Window<'a> {

    pub fn new(context: &'a Context) -> Self
    {
        Self {
            title: String::new(),
            origin: (0, 0),
            dimension: (0, 0),
            mapped: false,
            manager: Manager::None,
            context
        }
    }
}

impl<'a> Window<'a> {

    pub fn get_title(&self) -> &String
    {
        &self.title
    }

    pub fn get_origin(&self) -> Position
    {
        self.origin
    }

    pub fn get_dimension(&self) -> Dimension
    {
        self.dimension
    }

    pub(crate) fn get_manager(&self) -> &Manager
    {
        &self.manager
    }

    pub fn get_context(&self) -> &Context
    {
        &self.context
    }

    pub fn set_title(&mut self, title: &str)
    {
        self.title.clear();
        self.title.push_str(title);
    }

    pub fn set_origin(&mut self, origin: Position)
    {
        self.origin = origin;
    }

    pub fn set_dimension(&mut self, dim: Dimension)
    {
        self.dimension = dim;
    }

    pub(crate) fn set_manager(&mut self, manager: Manager)
    {
        self.manager = manager;
    }

    pub fn get_mapped(&self) -> bool
    {
        self.mapped
    }

    pub(crate) fn set_mapped(&mut self, mapped: bool)
    {
        self.mapped = mapped;
    }
}

impl<'a> Drop for Window<'a> {

    fn drop(&mut self)
    {
        if self.get_mapped() {
            ::unmap(self);
        }
    }
}
