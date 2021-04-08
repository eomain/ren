
extern crate ren;

use ren::{graphics::{cairo, Surface}, WindowCommand::*};

fn main()
{
    // Open a connection
    let mut connect = ren::Connection::open().unwrap();
    let token = connect.begin();

    connect.requests(&token, &[
        // Request the window title
        Title(format!("Ren - {}", file!())),
        // Request the window dimensions
        Dimension((300, 300)),
        // Map the window
        Map
    ]);

    // Create cairo context from window connection
    let surface = Surface::window(&mut connect, &token, (300, 300)).unwrap();
    let wx = surface.create_cairo_context();
    // Create cairo drawing buffer
    let buffer = Surface::buffer(&mut connect, &token, (300, 300)).unwrap();
    let cx = buffer.create_cairo_context();

    // Create cairo image surface
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/rust.png");
    let image = cairo::png_surface(path).unwrap();

    loop {
        // Wait for an event
        match connect.wait(&token).unwrap() {
            // Terminate response
            ren::Event::Terminate => break,
            ren::Event::Display(ren::DisplayEvent::Expose(map)) => {
                let (w, h) = map.1;
                // Draw background to buffer
                cx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                cx.rectangle(0.0, 0.0, w as f64, h as f64);
                cx.fill();

                // Draw image to buffer
                cx.set_source_surface(&image, 0.0, 0.0);
                cx.paint();
                // Draw buffer to window
                wx.set_source_surface(buffer.as_cairo_surface(), 0.0, 0.0);
                wx.paint();

                // Update window
                connect.request(&token, Update);
            },
            _ => ()
        }
    }
}
