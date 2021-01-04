
extern crate ren;

use ren::{
    graphics::cairo,
    XcbStat::{Connection, Window, VisualType},
    Data, XcbData, Body, Message,
    WindowCommand::{Title, Dimension, Map, Update},
};
use crate::cairo::{FontSlant, FontWeight, ImageSurface};

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

    // Create cairo context from window connection
    let surface = cairo::xcb_surface(&mut connect, &token, dim.0 as i32, dim.1 as i32).unwrap();
    let cx = cairo::Context::new(&surface);

    // Create cairo image surface
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/rust.png");
    let image = cairo::png_surface(path).unwrap();

    loop {
        // Wait for an event
        let message = connect.wait(&token).unwrap();

        match message.body() {
            // Terminate response
            Body::Event(ren::Event::Terminate) => break,
            Body::Event(ren::Event::Display(ren::DisplayEvent::Expose(map))) => {
                // Resize the surface
                surface.set_size(map.1.0 as i32, map.1.1 as i32);

                // Draw background
                cx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                cx.rectangle(0.0, 0.0, map.1.0 as f64, map.1.1 as f64);
                cx.fill();

                // Draw image
                cx.set_source_surface(&image, 0.0, 0.0);
                cx.paint();

                // Update window
                connect.request(&token, Update);
            },
            _ => ()
        }
    }
}
