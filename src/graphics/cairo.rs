
use std::fs::File;
use std::path::Path;
use crate::{Token, Data, Body};

pub use cairo::*;

/// Get cairo xcb surface from `Connection`
pub fn xcb_surface(connect: &mut crate::Connection,
           token: &Token, width: i32, height: i32) -> Option<XCBSurface>
{
    use crate::{XcbData, XcbStat::*};

    let id = match connect.request(&token, Window).ok()?.take_body() {
        Body::Data(Data::Xcb(XcbData::Window(id))) => id,
        _ => return None
    };

    let conn = match connect.request(&token, Connection).ok()?.take_body() {
        Body::Data(Data::Xcb(XcbData::Connection(conn))) => conn,
        _ => return None
    };

    let mut visual = match connect.request(&token, VisualType).ok()?.take_body() {
        Body::Data(Data::Xcb(XcbData::VisualType(v))) => v?,
        _ => return None
    };

    let conn = unsafe { XCBConnection::from_raw_none(conn.cast()) };
    let draw = XCBDrawable(id);
    let visual: *mut _ = &mut visual.base;
    let visual = unsafe { XCBVisualType::from_raw_none(visual.cast()) };
    XCBSurface::create(&conn, &draw, &visual, width, height).ok()
}

/// Get cairo png surface
pub fn png_surface<P>(path: P) -> Option<ImageSurface>
    where P: AsRef<Path>
{
    let mut png = File::open(path).ok()?;
    ImageSurface::create_from_png(&mut png).ok()
}
