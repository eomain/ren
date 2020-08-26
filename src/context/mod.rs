
pub mod xcb;

use crate::{
    Stat,
    Data,
    Window,
    render::Surface,
    event::Event
};

pub(crate) trait DisplayContext {

    fn init(_: &Window) -> Self;

    fn map(&self);

    fn unmap(&self);

    fn event(&self) -> Event;

    fn stat(&self, _: Stat) -> Option<Data>;

    fn draw(&self, _: &Surface);

    fn refresh(&self);
}
