extern crate ren;

use ren::WindowCommand::*;

fn main()
{
    let title = format!("Ren - example {}", file!());

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
                                println!("mouse-move: x: {}, y: {}", pos.0, pos.1)
                            }

                            ren::MouseEvent::Enter(pos) => {
                                println!("mouse-enter: x: {}, y: {}", pos.0, pos.1)
                            }

                            ren::MouseEvent::Leave(pos) => {
                                println!("mouse-leave: x: {}, y: {}", pos.0, pos.1)
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
