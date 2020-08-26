
extern crate xcb;

use super::DisplayContext;
use crate::display::window::Window;
use crate::display::Manager;
use crate::render;
use crate::render::Surface;
use crate::render::Font;
use crate::render::Point;
use crate::render::Line;
use crate::render::Rect;
use crate::event;
use crate::event::Event;
use crate::event::InputEvent;
use crate::event::DisplayEvent;
use crate::event::input::KeyEvent;
use crate::event::input::MouseEvent;
use crate::{
    Stat,
    Data,
    XcbStat,
    XcbData
};

pub(crate) fn init(context: &mut crate::Context)
{
    context.map = Some(map);
    context.unmap = Some(unmap);
    context.draw = Some(draw);
    context.event = Some(event);
    context.stat = Some(stat);
}

fn map(manager: &Manager)
{
    match manager.xcb() {
        Some(context) => context.map(),
        _ => ()
    }
}

fn unmap(manager: &Manager)
{
    match manager.xcb() {
        Some(context) => context.unmap(),
        _ => ()
    }
}

fn draw(manager: &Manager, surface: &render::Surface)
{
    match manager.xcb() {
        Some(context) => {
            context.draw(surface);
            context.refresh();
        },
        _ => ()
    }
}

fn event(manager: &Manager) -> Event
{
    match manager.xcb() {
        Some(context) => context.event(),
        _ => Event::None
    }
}

fn stat(manager: &Manager, stat: Stat) -> Option<Data>
{
    match manager.xcb() {
        Some(context) => context.stat(stat),
        _ => None
    }
}

pub struct Context {
    connection: xcb::Connection,
    window: xcb::Window,
    foreground: u32
}

const FONT_BASE: i16 = 10;

fn font(context: &Context, font: &Font)
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

fn point(context: &Context, point: &Point)
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

fn line(context: &Context, line: &Line)
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

fn rect(context: &Context, rect: &Rect)
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

static EVENT_MASK: xcb::EventMask = (
    xcb::EVENT_MASK_EXPOSURE |
    xcb::EVENT_MASK_KEY_PRESS |
    xcb::EVENT_MASK_BUTTON_PRESS |
    xcb::EVENT_MASK_BUTTON_RELEASE |
    xcb::EVENT_MASK_POINTER_MOTION |
    xcb::EVENT_MASK_BUTTON_MOTION |
    xcb::EVENT_MASK_BUTTON_1_MOTION |
    xcb::EVENT_MASK_BUTTON_2_MOTION |
    xcb::EVENT_MASK_BUTTON_3_MOTION |
    xcb::EVENT_MASK_BUTTON_4_MOTION |
    xcb::EVENT_MASK_BUTTON_5_MOTION |
    xcb::EVENT_MASK_ENTER_WINDOW |
    xcb::EVENT_MASK_LEAVE_WINDOW
);

impl DisplayContext for Context {

    fn init(window: &Window) -> Self
    {
        let (conn, num) = xcb::Connection::connect(None).unwrap();

        let (id, fore) = {
            let setup = conn.get_setup();
            let screen = setup.roots().nth(num as usize).unwrap();

            let fore = conn.generate_id();

            xcb::create_gc(
                &conn, fore, screen.root(), &[
                    (xcb::GC_FOREGROUND,
                     screen.white_pixel()),
                    (xcb::GC_GRAPHICS_EXPOSURES,
                     0)
                ]
            );

            let id = conn.generate_id();

            let values = [
                (xcb::CW_BACK_PIXEL, screen.black_pixel()),
                (xcb::CW_EVENT_MASK, EVENT_MASK)
            ];

            let (x, y) = window.origin();
            let (width, height) = window.dimension();
            let border = 10;

            xcb::create_window(
                &conn,
                xcb::COPY_FROM_PARENT as u8,
                id,
                screen.root(),
                x as i16,
                y as i16,
                width as u16,
                height as u16,
                border,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(),
                &values
            );

            xcb::change_property(
                &conn,
                xcb::PROP_MODE_REPLACE as u8,
                id,
                xcb::ATOM_WM_NAME,
                xcb::ATOM_STRING,
                8,
                window.title().as_bytes()
            );

            (id, fore)
        };

        Self {
            connection: conn,
            window: id,
            foreground: fore
        }
    }

    fn map(&self)
    {
        xcb::map_window(&self.connection, self.window);
        self.connection.flush();
    }

    fn unmap(&self)
    {
        xcb::unmap_window(&self.connection, self.window);
    }

    fn event(&self) -> Event
    {
        match self.connection.wait_for_event() {
            None => Event::Terminate,
            Some(e) => {
                let resp = e.response_type() & !0x080;

                match resp {
                    xcb::EXPOSE => {
                        Event::Display(
                            DisplayEvent::Expose(
                                event::xcb::expose(&e)
                            )
                        )
                    },

                    xcb::KEY_PRESS => {
                        Event::Input(
                            InputEvent::Key(
                                KeyEvent::Press(
                                    event::xcb::key_press(&e)
                                )
                            )
                        )
                    },

                    xcb::KEY_RELEASE => {
                        Event::Input(
                            InputEvent::Key(
                                KeyEvent::Release(
                                    event::xcb::key_release(&e)
                                )
                            )
                        )
                    },

                    xcb::BUTTON_PRESS => {
                        Event::Input(
                            InputEvent::Mouse(
                                MouseEvent::Press(
                                event::xcb::button_press(&e)
                                )
                            )
                        )
                    },

                    xcb::BUTTON_RELEASE => {
                        Event::Input(
                            InputEvent::Mouse(
                                MouseEvent::Release(
                                    event::xcb::button_release(&e)
                                )
                            )
                        )
                    },

                    xcb::MOTION_NOTIFY => {
                        Event::Input(
                            InputEvent::Mouse(
                                MouseEvent::Move(
                                    event::xcb::mouse_move(&e)
                                )
                            )
                        )
                    },

                    xcb::ENTER_NOTIFY => {
                        Event::Input(
                            InputEvent::Mouse(
                                MouseEvent::Enter(
                                    event::xcb::mouse_enter(&e)
                                )
                            )
                        )
                    },

                    xcb::LEAVE_NOTIFY => {
                        Event::Input(
                            InputEvent::Mouse(
                                MouseEvent::Leave(
                                    event::xcb::mouse_leave(&e)
                                )
                            )
                        )
                    },

                    _ => Event::None
                }
            }
        }
    }

    fn stat(&self, status: Stat) -> Option<Data>
    {
        match status {
            Stat::Xcb(status) => {
                Some((match status {
                    XcbStat::Connection => XcbData::Connection(self.connection.get_raw_conn()),
                    XcbStat::Window => XcbData::Window(self.window)
                }).into())
            },
            _ => None
        }
    }

    fn draw(&self, surface: &Surface)
    {
        surface.for_each(|object| {
            use crate::render::Object;
            use crate::render::Primitive;

            match &*object {
                Object::Primitive(p) => match p {
                    Primitive::Text(ref f) => {
                        font(self, f);
                    },

                    Primitive::Point(ref p) => {
                        point(self, p);
                    },

                    Primitive::Line(ref l) => {
                        line(self, l);
                    },

                    Primitive::Rect(ref r) => {
                        rect(self, r);
                    }
                },
                _ => ()
            }
        });
    }

    fn refresh(&self)
    {
        self.connection.flush();
    }
}

impl Drop for Context {

    fn drop(&mut self)
    {
        self.unmap();
    }
}
