
extern crate ren;

use ren::{render::context::Context, graphics::cairo::*, WindowCommand::*};

fn main()
{
    // Open a connection
    let mut connect = ren::Connection::open().unwrap();
    let token = connect.begin();

    // Request the window title
    connect.request(&token, Title(format!("Ren - {}", file!())));

    // Request the window dimensions
    connect.request(&token, Dimension((300, 300)));

    // Map the window
    connect.request(&token, Map);

    // Create surface
    let surface = xcb_surface(&mut connect, &token, 300, 300).unwrap();

    // Maintain the context state
    let mut state = Some(State::new());
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/rust.png");

    loop {
        // Wait for an event
        match connect.wait(&token).unwrap() {
            // Terminate response
            ren::Event::Terminate => break,
            ren::Event::Display(ren::DisplayEvent::Expose(map)) => {
                let (w, h) = map.1;
                // Resize the surface
                surface.set_size(w as i32, h as i32);

                let mut cx = Context::new();

                // Draw background
                cx.rgba(1.0, 1.0, 1.0, 1.0);
                cx.rect((0, 0), w as usize, h as usize);
                cx.fill();

                // Draw image
                cx.image(path, (0, 0));
                cx.paint();

                // Render to surface
                state = render(&cx, &surface, state);

                // Update window
                connect.request(&token, Update);
            },
            _ => ()
        }
    }
}
