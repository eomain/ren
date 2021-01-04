extern crate ren;

use ren::WindowCommand::*;
use ren::async_std::task;

fn main()
{
    task::block_on(async {
        let title = format!("Ren - {}", file!());

        // Open a connection
        let mut connect = ren::Connection::new();
        let token = connect.begin();

        // Request the window title
        connect.request(&token, Title(title));

        // Request the window dimensions
        connect.request(&token, Dimension((640, 480)));

        // Map the window
        connect.request(&token, Map);
        connect.request(&token, Update);

        loop {
            // Wait for an event
            let message = connect.session(&token).unwrap().await;
            println!("{:?}", message);

            match message.body() {
                ren::Body::Event(ren::Event::Terminate) => break,
                _ => ()
            }
        }
    });
}
