extern crate xcb;

use render;

pub fn point(point: &render::Point) -> xcb::Point
{
    let (x, y) = point.into();
    xcb::Point::new(x as i16, y as i16)
}

pub fn line(line: &render::Line) -> Vec<xcb::Point>
{
    line.points().iter().map(|p| point(p)).collect()
}

pub fn rectangle(rect: &render::Rectangle) -> xcb::Rectangle
{
    let (point, w, h) = rect.into();
    let (x, y) = point.into();

    xcb::Rectangle::new(
        x as i16,
        y as i16,
        w as u16,
        h as u16
    )
}

pub fn font<'a>(font: &'a render::Font) -> (i16, i16, &'a str)
{
    let (x, y) = font.get_point().into();
    (x as i16, y as i16, font.get_text())
}
