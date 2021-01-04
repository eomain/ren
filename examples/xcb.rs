
extern crate ren;

use ren::{
    XcbStat::{Connection, Window, VisualType},
    Data, XcbData, Body, Message,
    WindowCommand::{Title, Dimension, Map, Update},
};

fn main()
{
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

    // Get the window ID
    let id = match connect.request(&token, Window).unwrap().body() {
        Body::Data(Data::Xcb(XcbData::Window(id))) => *id,
        _ => unreachable!()
    };

    // Get the connection
    let conn = match connect.request(&token, Connection).unwrap().body() {
        Body::Data(Data::Xcb(XcbData::Connection(conn))) => *conn,
        _ => unreachable!()
    };

    let mut visual = match connect.request(&token, VisualType).unwrap().body() {
        Body::Data(Data::Xcb(XcbData::VisualType(v))) => v.unwrap(),
        _ => unreachable!()
    };

    println!("XCB Id: {:?}", id);
    println!("XCB Connection: {:?}", conn);
    println!("XCB Visual Id: {}", visual.visual_id());

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
