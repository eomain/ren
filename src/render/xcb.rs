extern crate xcb;

use render;

pub fn point(point: &render::Point) -> xcb::Point
{
    xcb::Point::new(point.get_x() as i16, point.get_y() as i16)
}

pub fn line(line: &render::Line) -> Vec<xcb::Point>
{
    let mut points = Vec::new();
    for p in line.get() {
        points.push(point(p));
    }

    points
}

pub fn rectangle(rect: &render::Rectangle) -> xcb::Rectangle
{
    let (x, y) = rect.get_point().get();
    xcb::Rectangle::new(
        x as i16,
        y as i16,
        rect.get_width() as u16,
        rect.get_height() as u16
    )
}

pub fn font<'a>(font: &'a render::Font) -> (i16, i16, &'a str)
{
    let point = font.get_point();
    (point.get_x() as i16, point.get_y() as i16, font.get_text())
}
