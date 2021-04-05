
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::{Token, Data, Body, render::context};

pub use cairo::*;

/// Cairo surface
#[cfg(target_family = "unix")]
pub type Surface = XCBSurface;

/// Get a cairo window surface from a `Connection`
#[cfg(target_family = "unix")]
pub fn surface(connect: &mut crate::Connection, window: &Token,
    dimensions: (i32, i32)) -> Option<Surface>
{
    xcb_surface(connect, window, dimensions.0, dimensions.1)
}

/// Get cairo xcb surface from a `Connection`
#[cfg(target_family = "unix")]
fn xcb_surface(connect: &mut crate::Connection,
           token: &Token, width: i32, height: i32) -> Option<Surface>
{
    use crate::{data::XcbData, stat::XcbStat::*};

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

pub struct State {
    images: HashMap<PathBuf, ImageSurface>
}

impl State {
    pub fn new() -> Self {
        Self {
            images: HashMap::new()
        }
    }
}

pub fn render(cx: &context::Context, surface: &Surface, mut state: Option<State>) -> Option<State>
{
    use context::{Command, ImageType, ImageFormat};

    let cr = cairo::Context::new(&surface);

    use Command::*;
    for command in cx.commands() {
        match command {
            Rgb(red, green, blue) => cr.set_source_rgb(*red, *green, *blue),
            Rgba(red, green, blue, alpha) => cr.set_source_rgba(*red, *green, *blue, *alpha),
            Text(text) => cr.show_text(text),
            Image(point, ty) => match ty {
                ImageType::Path(path) => {
                    if let Some(state) = state.as_ref() {
                        if let Some(image) = state.images.get(path) {
                            cr.set_source_surface(&image, point.x as f64, point.y as f64);
                            continue;
                        }
                    }

                    if let Some("png") = path.extension().map(|p| p.to_str()).flatten() {
                        if let Some(png) = png_surface(path) {
                            cr.set_source_surface(&png, point.x as f64, point.y as f64);
                            if let Some(state) = state.as_mut() {
                                state.images.insert(path.to_path_buf(), png);
                            }
                        }
                    }
                },
                ImageType::Data(data, format, width, height) => {
                    let format = match format {
                        ImageFormat::Bgra8 => Format::ARgb32,
                        _ => continue
                    };
                    let mut data = match data.try_borrow_mut() {
                        Err(_) => continue,
                        Ok(data) => data.to_owned()
                    };
                    let stride = match format.stride_for_width(*width) {
                        Err(_) => continue,
                        Ok(stride) => stride
                    };
                    let width = *width as i32;
                    let height = *height as i32;
                    let image = ImageSurface::create_for_data(data, format, width, height, stride);
                    if let Ok(image) = image {
                        cr.set_source_surface(&image, point.x as f64, point.y as f64);
                    }
                }
            },
            FontSize(size) => cr.set_font_size(*size),
            Move(point) => cr.move_to(point.x as f64, point.y as f64),
            RelMove(point) => cr.rel_move_to(point.x as f64, point.y as f64),
            Line(point) => cr.line_to(point.x as f64, point.y as f64),
            RelLine(point) => cr.rel_line_to(point.x as f64, point.y as f64),
            Rect(rect) => {
                let (x, y) = (rect.point.x, rect.point.y);
                cr.rectangle(x as f64, y as f64, rect.width as f64, rect.height as f64);
            },
            RelRect(width, height) => {
                cr.rel_move_to(0.0, 0.0);
                cr.rel_line_to(*width as f64, 0.0);
                cr.rel_line_to(0.0, *height as f64);
                cr.rel_line_to(-(*width as f64), 0.0);
                cr.close_path();
            },
            Arc(point, radius, angle1, angle2) => {
                let (x, y) = (point.x as f64, point.y as f64);
                cr.arc(x, y, *radius as f64, *angle1, *angle2);
            },
            Curve(p1, p2, p3) => {
                let (x1, y1) = (p1.x as f64, p1.y as f64);
                let (x2, y2) = (p2.x as f64, p2.y as f64);
                let (x3, y3) = (p3.x as f64, p3.y as f64);
                cr.curve_to(x1, y1, x2, y2, x3, y3);
            },
            Scale(x, y) => cr.scale(*x, *y),
            Rotate(angle) => cr.rotate(*angle),
            Translate(x, y) => cr.translate(*x, *y),
            Stroke => cr.stroke(),
            Fill => cr.fill(),
            Paint => cr.paint(),
            _ => ()
        }
    }

    state
}
