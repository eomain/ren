
extern crate ren;

use ren::{graphics::cairo, WindowCommand::*};

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

    // Create cairo context from window connection
    let surface = cairo::xcb_surface(&mut connect, &token, 300, 300).unwrap();
    let cx = cairo::Context::new(&surface);

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
                // Resize the surface
                surface.set_size(w as i32, h as i32);

                // Draw background
                cx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                cx.rectangle(0.0, 0.0, w as f64, h as f64);
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
