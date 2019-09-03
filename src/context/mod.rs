
pub mod xcb;

use display::window::Window;
use render::Surface;
use event::Event;

pub trait DisplayContext {

    fn init(&Window) -> Self;

    fn map(&self);

    fn unmap(&self);

    fn event(&self) -> Event;

    fn draw(&self, &Surface);

    fn refresh(&self);
}
