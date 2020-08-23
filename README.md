
# Ren
A data-driven, event based library for communicating with the underlying
windowing system (X11, XCB, etc.) via a relatively stateless and extensible
messaging interface.
Messages exchanged between the client program and the display server, can
be either a request (e.g. map the window) or a response (e.g. a key-press).

## Features
- Easy to understand and simple API
- Flexible and extensible messaging interface
- Designed as a compact library
- Portable across platforms (UNIX, Linux, BSD, etc.) with X11 and/or XCB support
- Data-driven design eliminates complexity
- Has support for drawing graphics
- Receive events as messages
    - Display events (Expose / redraw event)
    - Input events
        - Key events (Press, Release)
        - Mouse events (Press, Release, Move, Enter, Leave)

## Example
```rust
extern crate ren;

use ren::Message;
use ren::WindowCommand::{
    Title, Dimension, Map
};

fn main()
{
    let title = format!("Ren - example {}", file!());

    // Open a connection
    let mut connect = ren::Connection::new();
    let token = connect.begin();

    // Request the window title
    connect.send(&token, Message::request(Title(title)));

    // Request the window dimensions
    connect.send(&token, Message::request(Dimension((640, 480))));

    // Map the window
    connect.send(&token, Message::request(Map));

    loop {
        // Wait for an event
        let message = connect.wait(&token).unwrap();
        println!("{:?}", message);

        match message.body() {
            // Terminate response
            ren::Body::Event(ren::Event::Terminate) => break,
            _ => ()
        }
    }
}
```

## License
Ren is distributed under the MIT license.
