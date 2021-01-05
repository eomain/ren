
extern crate ren;

use ren::{
    data::XcbData,
    stat::XcbStat::{Connection, Window, VisualType},
    Data, Body, Message,
    WindowCommand::{Title, Dimension, Map, Update},
};

fn main()
{
    let title = format!("Ren - {}", file!());

    // Open a connection
    let mut connect = ren::Connection::open().unwrap();
    let token = connect.begin();

    // Request the window title
    connect.request(&token, Title(title));

    // Request the window dimensions
    connect.request(&token, Dimension((640, 480)));

    // Map the window
    connect.request(&token, Map);
    connect.request(&token, Update);

    // Get the window ID
    let id = match connect.request(&token, Window).unwrap().take_body() {
        Body::Data(Data::Xcb(XcbData::Window(id))) => id,
        _ => unreachable!()
    };

    // Get the connection
    let conn = match connect.request(&token, Connection).unwrap().take_body() {
        Body::Data(Data::Xcb(XcbData::Connection(conn))) => conn,
        _ => unreachable!()
    };

    let mut visual = match connect.request(&token, VisualType).unwrap().take_body() {
        Body::Data(Data::Xcb(XcbData::VisualType(v))) => v.unwrap(),
        _ => unreachable!()
    };

    println!("XCB Id: {:?}", id);
    println!("XCB Connection: {:?}", conn);
    println!("XCB Visual Id: {}", visual.visual_id());

    loop {
        // Wait for an event
        let event = connect.wait(&token).unwrap();

        match event {
            // Terminate response
            ren::Event::Terminate => break,
            _ => ()
        }
    }
}
