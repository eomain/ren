extern crate ren;
extern crate mirage;

use mirage::convert::svg;
use ren::render::Surface;
use ren::WindowCommand::*;

fn surface() -> Surface
{
    svg::into::string(r#"
        <svg>
            <line x1="40" y1="10" x2="40" y2="30" />
            <line x1="40" y1="30" x2="60" y2="30" />
            <line x1="60" y1="30" x2="60" y2="10" />
            <line x1="60" y1="10" x2="40" y2="10" />
        </svg>
    "#).unwrap()
}

#[test]
fn main() {
    let title = format!("Ren - example");

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
        let message = connect.wait(&token).unwrap();
        println!("{:?}", message);

        match message.body() {
            // Terminate response
            ren::Body::Event(ren::Event::Terminate) => break,
            // Display response
            ren::Body::Event(ren::Event::Display(event)) => {
                // Expose response
                if let ren::DisplayEvent::Expose(map) = event {
                    let (x, y) = map.0;
                    let (w, h) = map.1;
                    println!("pos: ({}, {}), dim: ({}, {})", x, y, w, h);
                    // Draw on the window
                    connect.request(&token, Draw(surface()));
                    connect.request(&token, Update);
                }
            },
            // Input response
            ren::Body::Event(ren::Event::Input(event)) => {
                match event {
                    ren::InputEvent::Key(event) => {
                        match event {
                            ren::KeyEvent::Press(_) => {
                                println!("key-press");
                            },

                            ren::KeyEvent::Release(_) => {
                                println!("key-release");
                            }

                            _ => ()
                        }
                    },

                    ren::InputEvent::Mouse(event) => {
                        match event {
                            ren::MouseEvent::Press(pos) => {
                                println!("button-press: x: {}, y: {}", pos.0, pos.1)
                            },

                            ren::MouseEvent::Release(pos) => {
                                println!("button-release: x: {}, y: {}", pos.0, pos.1)
                            },

                            ren::MouseEvent::Move(pos) => {
                                println!("hover: x: {}, y: {}", pos.0, pos.1)
                            }

                            _ => ()
                        }
                    },

                    _ => ()
                }
            }
            _ => ()
        }
    }
}
