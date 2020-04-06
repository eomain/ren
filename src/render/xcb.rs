extern crate xcb;

use super::*;

pub fn point(point: &Point) -> xcb::Point
{
    let (x, y) = point.into();
    xcb::Point::new(x as i16, y as i16)
}

pub fn line(line: &Line) -> Vec<xcb::Point>
{
    line.path().iter().map(|p| point(p)).collect()
}

pub fn rectangle(rect: &Rect) -> xcb::Rectangle
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

pub fn font<'a>(font: &'a Font) -> (i16, i16, &'a str)
{
    let p = &font.point;
    let (x, y) = p.into();
    (x as i16, y as i16, &font.text)
}
