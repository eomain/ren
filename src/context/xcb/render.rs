
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

pub struct Image {
    pixmap: xcb::Pixmap
}

impl Image {
    pub fn new(context: &super::Context, draw: xcb::Drawable, width: u16, height: u16) -> Self
    {
        let pixmap = context.id();
        xcb::create_pixmap(
            &context.connection,
            xcb::COPY_FROM_PARENT as u8,
            pixmap,
            draw,
            width,
            height
        );

        Self {
            pixmap
        }
    }

    pub fn from_window(context: &super::Context, width: u16, height: u16) -> Self
    {
        Self::new(context, context.window, width, height)
    }

    pub fn write(&self, context: &super::Context, x: i16, y: i16,
                 width: u16, height: u16, data: &[u8])
    {
        xcb::put_image(
            &context.connection,
            xcb::IMAGE_FORMAT_XY_PIXMAP as u8,
            self.pixmap,
            context.foreground,
            width, height,
            x, y,
            0,
            xcb::COPY_FROM_PARENT as u8,
            data
        );
    }

    pub fn draw(&self, context: &super::Context, draw: xcb::Drawable, src: (i16, i16),
                dest: (i16, i16), width: u16, height: u16)
    {
        xcb::copy_area(
            &context.connection,
            self.pixmap,
            draw,
            context.foreground,
            src.0, src.1,
            dest.0, dest.1,
            width, height
        );
    }

    pub fn draw_window(&self, context: &super::Context, src: (i16, i16), dest: (i16, i16),
                       width: u16, height: u16)
    {
        self.draw(context, context.window, src, dest, width, height);
    }
}
