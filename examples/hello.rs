extern crate ren;
extern crate mirage;

use mirage::convert::svg;
use ren::render::Surface;

fn surface() -> Surface
{
    svg::into::string(r#"
        <svg>
            <text x="280" y="150">hello world</text>
        </svg>
    "#).unwrap()
}

fn main()
{
    let title = format!("Ren - example {}", file!());

    // Open a connection
    let mut connect = ren::Connection::new();
    let token = connect.begin();

    // Request the window title
    connect.send(&token, ren::Message::request(
        ren::WindowCommand::Title(title)
    ));

    // Request the window dimensions
    connect.send(&token, ren::Message::request(
        ren::WindowCommand::Dimension((640, 480))
    ));

    // Map the window
    connect.send(&token, ren::Message::request(
        ren::WindowCommand::Map
    ));

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
                    connect.send(&token, ren::Message::request(
                        ren::WindowCommand::Draw(surface())
                    ));
                }
            },
            _ => ()
        }
    }
}
