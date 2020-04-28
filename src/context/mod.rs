
pub mod xcb;

use crate::{
    Window,
    render::Surface,
    event::Event
};

pub(crate) trait DisplayContext {

    fn init(_: &Window) -> Self;

    fn map(&self);

    fn unmap(&self);

    fn event(&self) -> Event;

    fn draw(&self, _: &Surface);

    fn refresh(&self);
}
