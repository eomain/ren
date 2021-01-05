
extern crate ren;

use ren::{async_std::task, WindowCommand::*};

fn main()
{
    task::block_on(async {
        // Open a connection
        let mut connect = ren::Connection::open().unwrap();
        // Create window session
        let session = connect.begin();

        connect.requests(&session, &[
            // Request the window title
            Title(format!("Ren - {}", file!())),
            // Request the window dimensions
            Dimension((640, 480)),
            // Map the window
            Map,
            // Update the window
            Update
        ]);

        loop {
             // Await the event
             let event = connect.event(&session).await.unwrap();
             println!("{:?}", event);

             match event {
                 // Terminate application
                 ren::Event::Terminate => break,
                 _ => ()
             }
         }
    });
}
