
extern crate xcb;

use crate::{
    render,
    render::{
        Font,
        Point,
        Line,
        Rect,
        Surface
    },
    event
};
use super::Context;

const FONT_BASE: i16 = 10;

pub fn font(context: &Context, font: &Font)
{
    let fid = context.connection.generate_id();
    xcb::open_font(&context.connection, fid, "fixed");

    let (x, y, text) = render::xcb::font(font);
    xcb::image_text_8(
        &context.connection,
        context.window,
        context.foreground,
        x,
        y + FONT_BASE,
        text
    );

    xcb::close_font(&context.connection, fid);
}

pub fn point(context: &Context, point: &Point)
{
    let point = &[
        render::xcb::point(point)
    ];

    xcb::poly_point(
        &context.connection,
        xcb::COORD_MODE_ORIGIN as u8,
        context.window,
        context.foreground,
        point
    );
}

pub fn line(context: &Context, line: &Line)
{
    let line = render::xcb::line(line);

    xcb::poly_line(
        &context.connection,
        xcb::COORD_MODE_ORIGIN as u8,
        context.window,
        context.foreground,
        &line
    );
}

pub fn rect(context: &Context, rect: &Rect)
{
    let rect = &[
        render::xcb::rectangle(rect)
    ];

    xcb::poly_rectangle(
        &context.connection,
        context.window,
        context.foreground,
        rect
    );
}
