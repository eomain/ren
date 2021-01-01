
mod context;
mod render;

use super::SystemContext;
pub use context::Context;

pub fn init(context: &mut super::Context)
{
    context.event = Box::new(|m| m.xcb_unwrap().event());
    context.poll = Box::new(|m| m.xcb_unwrap().poll());
    context.stat = Box::new(|m, s| m.xcb_unwrap().stat(s));
    context.window = Box::new(|m, c| m.xcb_unwrap().window(c));
}
