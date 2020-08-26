extern crate ren;

use ren::{
    XcbStat::{
        Connection,
        Window
    },
    Data,
    XcbData,
    Body,
    Message,
    WindowCommand::{
        Title, Dimension, Map
    },
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
    connect.send(&token, Message::request(Dimension((320, 240))));

    // Map the window
    connect.send(&token, Message::request(Map));

    // Get the window ID
    let id = match connect.send(&token, Message::request(Window)).unwrap().body() {
        Body::Data(Data::Xcb(XcbData::Window(id))) => *id,
        _ => unreachable!()
    };

    // Get the connection
    let conn = match connect.send(&token, Message::request(Connection)).unwrap().body() {
        Body::Data(Data::Xcb(XcbData::Connection(conn))) => *conn,
        _ => unreachable!()
    };

    println!("XCB ID: {:?}", id);
    println!("XCB connection: {:?}", conn);

    loop {
        // Wait for an event
        let message = connect.wait(&token).unwrap();

        match message.body() {
            // Terminate response
            Body::Event(ren::Event::Terminate) => break,
            _ => ()
        }
    }
}
