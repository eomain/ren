
pub mod xcb;

use crate::display::window::Window;
use crate::render::Surface;
use crate::event::Event;

pub trait DisplayContext {

    fn init(&Window) -> Self;

    fn map(&self);

    fn unmap(&self);

    fn event(&self) -> Event;

    fn draw(&self, &Surface);

    fn refresh(&self);
}
