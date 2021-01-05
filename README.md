
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
- Has support for drawing vector graphics
- Receive events as messages
    - Display events (Expose / redraw event)
    - Input events
        - Key events (Press, Release)
        - Mouse events (Press, Release, Move, Enter, Leave)

## Example
```rust
use ren::WindowCommand::*;

// Open a connection
let mut connect = ren::Connection::open().unwrap();

// Create window session
let session = connect.begin();

// Request the window title
connect.request(&session, Title(format!("Ren - example")));

// Request the window dimensions
connect.request(&session, Dimension((320, 240)));

// Map the window
connect.request(&session, Map);

// Update the window
connect.request(&session, Update);

loop {
    // Wait for an event
    let event = connect.wait(&session).unwrap();
    println!("{:?}", event);

    match event {
        // Terminate application
        ren::Event::Terminate => break,
        _ => ()
    }
}
```

## License
Ren is distributed under the MIT license.
