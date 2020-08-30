
pub mod xcb;

use crate::{
    Stat,
    Data,
    WindowCommand,
    render::Surface,
    event::Event
};

pub(crate) trait DisplayContext {

    fn init() -> Self;

    fn event(&self) -> Event;

    fn stat(&self, _: Stat) -> Option<Data>;

    fn window(&self, _: &WindowCommand);

    fn update(&self);
}
