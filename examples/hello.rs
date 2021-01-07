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
        Dimension((640, 480)),
        // Map the window
        Map
    ]);

    // Create surface
    let surface = surface(&mut connect, &token, (300, 300)).unwrap();

    loop {
        // Wait for an event
        let event = connect.wait(&token).unwrap();
        println!("{:?}", event);

        match event {
            // Terminate response
            ren::Event::Terminate => break,
            // Display response
            ren::Event::Display(ren::DisplayEvent::Expose(map)) => {
                let (w, h) = map.1;
                // Resize the surface
                surface.set_size(w as i32, h as i32);

                let mut cx = Context::new();

                // Draw background
                cx.rgba(1.0, 1.0, 1.0, 1.0);
                cx.rect((0, 0), w as usize, h as usize);
                cx.fill();

                cx.rgb(0.0, 0.0, 0.0);
                cx.move_to((280, 150));
                cx.text("hello world");

                // Render to surface
                render(&cx, &surface, None);

                // Update window
                connect.request(&token, Update);
            },
            _ => ()
        }
    }
}
