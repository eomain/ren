extern crate ren;
extern crate mirage;

use mirage::convert::svg;
use ren::render::Surface;
use ren::WindowCommand::*;

fn surface() -> Surface
{
    svg::into::string(r#"
        <svg>
            <line x1="0" y1="0" x2="640" y2="480" />
            <rect width="639" height="479" />
            <line x1="640" y1="0" x2="0" y2="480" />
        </svg>
    "#).unwrap()
}

fn main()
{
    let title = format!("Ren - example {}", file!());

    // Open a connection
    let mut connect = ren::Connection::new();
    let token = connect.begin();

    // Create a message queue
    let mut queue = ren::MessageQueue::new();

    // Request the window title
    queue.enqueue(ren::Message::request(Title(title)));
    // Request the window dimensions
    queue.enqueue(ren::Message::request(Dimension((640, 480))));
    // Map the window
    queue.enqueue(ren::Message::request(Map));
    queue.enqueue(ren::Message::request(Update));

    // Append to the connection message queue
    let batch = connect.batch(&token, queue).unwrap();
    // Clear out the message queue
    connect.dispatch(&token, &batch);

    loop {
        // Wait for an event
        let message = connect.wait(&token).unwrap();
        println!("{:?}", message);

        match message.body() {
            // Terminate response
            ren::Body::Event(ren::Event::Terminate) => break,
            // Display response
            ren::Body::Event(ren::Event::Display(event)) => {
                // Expose response
                if let ren::DisplayEvent::Expose(_) = event {
                    // Draw on the window
                    connect.request(&token, Draw(surface()));
                    connect.request(&token, Update);
                }
            },
            _ => ()
        }
    }
}
