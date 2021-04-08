
extern crate ren;

use ren::{graphics::{context::Context, *}, WindowCommand::*};

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

    // Create a surface for the window
    let mut surface = Surface::window(&mut connect, &token, (300, 300)).unwrap();
    // Create a drawing buffer
    let buffer = Surface::buffer(&mut connect, &token, (300, 300)).unwrap();

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/rust.png");

    loop {
        // Wait for an event
        match connect.wait(&token).unwrap() {
            // Terminate response
            ren::Event::Terminate => break,
            ren::Event::Display(ren::DisplayEvent::Expose(map)) => {
                let (w, h) = map.1;
                let mut cx = Context::new();

                // Draw background
                cx.rgba(1.0, 1.0, 1.0, 1.0);
                cx.rect((0, 0), w as usize, h as usize);
                cx.fill();

                // Draw image
                cx.image(path, (0, 0));
                cx.paint();
                
                // Draw to the buffer
                buffer.render(&cx);

                // Render to the window surface
                surface.copy(&buffer);

                // Update window
                connect.request(&token, Update);
            },
            _ => ()
        }
    }
}
