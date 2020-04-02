//! # Ren

//! `ren` is library that provides basic access
//! to the client windowing system. There is
//! support for rendering primative drawing operations to
//! the window surface.

extern crate xcb;
extern crate mirage;

mod context;
mod display;

pub mod event;
pub mod render;

pub mod prelude;
pub use crate::prelude::*;

use crate::display::Manager;
use crate::display::ManagerName;
use crate::render::Surface;

pub struct Context {
    name: ManagerName,
    map: Option<fn(&Manager)>,
    unmap: Option<fn(&Manager)>,
    draw: Option<fn(&Manager, &render::Surface)>,
    event: Option<fn(&Manager) -> Event>
}

impl Context {

    fn new(name: ManagerName) -> Self
    {
        Self {
            name,
            map: None,
            unmap: None,
            draw: None,
            event: None
        }
    }
}

impl Context {

    fn init(&mut self)
    {
        self.get_name().init(self);
    }

    fn get_name(&self) -> ManagerName
    {
        self.name.clone()
    }

    fn set_map(&mut self, func: fn(&Manager))
    {
        self.map = Some(func);
    }

    fn set_unmap(&mut self, func: fn(&Manager))
    {
        self.unmap = Some(func);
    }

    fn set_draw(&mut self, func: fn(&Manager, &render::Surface))
    {
        self.draw = Some(func);
    }

    fn set_event(&mut self, func: fn(&Manager) -> Event)
    {
        self.event = Some(func);
    }
}

impl Drop for Context {

    fn drop(&mut self)
    {

    }
}

pub fn init() -> Result<Context, ()>
{

    let mut context = Context::new(
        ManagerName::default()
    );

    context.init();

    if let ManagerName::None = context.get_name() {
        Err(())
    } else {
        Ok(context)
    }
}

pub fn map(window: &mut Window)
{
    if !window.get_mapped() {
        let name = window.get_context().get_name();
        Manager::init(&name, window);
        window.set_mapped(true);
    }

    let context = window.get_context();

    if let Some(map) = context.map {
        map(window.get_manager());
    }
}

pub fn unmap(window: &mut Window)
{
    {
        let context = window.get_context();

        if let Some(unmap) = context.unmap {
            unmap(window.get_manager());
        }
    }

    window.set_mapped(false);
}

pub fn draw(window: &Window, surface: &Surface)
{
    let context = window.get_context();

    if let Some(draw) = context.draw {
        draw(window.get_manager(), surface);
    }
}

pub fn events<F>(window: &Window, event_loop: F)
    where F: Fn(Event)
{
    let context = window.get_context();

    if let Some(event) = context.event {
        loop {
            let event = event(window.get_manager());

            let term = match event {
                Event::None => continue,
                Event::Terminate => true,
                _ => false
            };

            event_loop(event);

            if term {
                return;
            }
        }
    }
}
