
mod context;
mod render;

use super::DisplayContext;
pub use context::Context;

pub fn init(context: &mut super::Context)
{
    context.event = Box::new(|m| m.xcb_unwrap().event());
    context.stat = Box::new(|m, s| m.xcb_unwrap().stat(s));
    context.window = Box::new(|m, c| m.xcb_unwrap().window(c));
}
