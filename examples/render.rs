
extern crate ren;

use ren::{
    render::context::Context,
    graphics::cairo::{State, xcb_surface, render},
    Body,
    WindowCommand::{Title, Dimension, Map, Update},
};

fn main()
{
    let title = format!("Ren - {}", file!());
    let dim = (300, 300);

    // Open a connection
    let mut connect = ren::Connection::new();
    let token = connect.begin();

    // Request the window title
    connect.request(&token, Title(title));

    // Request the window dimensions
    connect.request(&token, Dimension(dim));

    // Map the window
    connect.request(&token, Map);

    // Create surface
    let surface = xcb_surface(&mut connect, &token, dim.0 as i32, dim.1 as i32).unwrap();

    // Maintain the context state
    let mut state = Some(State::new());
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/rust.png");

    loop {
        // Wait for an event
        let message = connect.wait(&token).unwrap();

        match message.body() {
            // Terminate response
            Body::Event(ren::Event::Terminate) => break,
            Body::Event(ren::Event::Display(ren::DisplayEvent::Expose(map))) => {
                // Resize the surface
                surface.set_size(map.1.0 as i32, map.1.1 as i32);

                let mut cx = Context::new();

                // Draw background
                cx.rgba(1.0, 1.0, 1.0, 1.0);
                cx.rect((0, 0), map.1.0 as usize, map.1.1 as usize);
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
