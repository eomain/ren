
extern crate xcb;

use super::DisplayContext;
use crate::display::Manager;
use crate::render;
use crate::render::Surface;
use crate::render::Font;
use crate::render::Point;
use crate::render::Line;
use crate::render::Rect;
use crate::event;
use crate::{
    Stat,
    Data,
    XcbStat,
    XcbData,
    WindowCommand,
    Event,
    InputEvent,
    DisplayEvent,
    KeyEvent,
    MouseEvent
};

pub(crate) fn init(context: &mut crate::Context)
{
    context.event = Some(event);
    context.stat = Some(stat);
    context.window = Box::new(window);
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

fn window(manager: &Manager, command: &WindowCommand)
{
    match manager.xcb() {
        Some(context) => context.window(command),
        _ => ()
    }
}

pub struct Context {
    connection: xcb::Connection,
    window: xcb::Window,
    foreground: u32
}

impl Context {

    fn property(&self, mode: xcb::PropMode, prop: xcb::AtomEnum, ty: xcb::AtomEnum, data: &[u8])
    {
        xcb::change_property(
            &self.connection,
            mode as u8,
            self.window,
            prop,
            ty,
            8,
            data
        );
    }

    fn configure(&self, values: &[(u16, u32)])
    {
        xcb::configure_window(
            &self.connection,
            self.window,
            values
        );
    }

    fn window_title(&self, name: &str)
    {
        use xcb::*;
        self.property(PROP_MODE_REPLACE, ATOM_WM_NAME, ATOM_STRING, name.as_bytes());
    }

    fn window_icon_title(&self, name: &str)
    {
        use xcb::*;
        self.property(PROP_MODE_REPLACE, ATOM_WM_ICON_NAME, ATOM_STRING, name.as_bytes());
    }

    fn window_x(&self, x: u32)
    {
        self.configure(&[(
            xcb::CONFIG_WINDOW_X as u16, x
        )]);
    }

    fn window_y(&self, y: u32)
    {
        self.configure(&[(
            xcb::CONFIG_WINDOW_Y as u16, y
        )]);
    }

    fn window_move(&self, x: u32, y: u32)
    {
        self.configure(&[
            (xcb::CONFIG_WINDOW_X as u16, x),
            (xcb::CONFIG_WINDOW_Y as u16, y)
        ]);
    }

    fn window_width(&self, width: u32)
    {
        self.configure(&[(
            xcb::CONFIG_WINDOW_WIDTH as u16, width
        )]);
    }

    fn window_height(&self, height: u32)
    {
        self.configure(&[(
            xcb::CONFIG_WINDOW_HEIGHT as u16, height
        )]);
    }

    fn window_resize(&self, width: u32, height: u32)
    {
        self.configure(&[
            (xcb::CONFIG_WINDOW_WIDTH  as u16, width),
            (xcb::CONFIG_WINDOW_HEIGHT as u16, height)
        ]);
    }

    fn window_map(&self)
    {
        xcb::map_window(&self.connection, self.window);
        self.update();
    }

    fn window_unmap(&self)
    {
        xcb::unmap_window(&self.connection, self.window);
    }

    fn window_stack_above(&self)
    {
        self.configure(&[(
            xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE
        )]);
    }

    fn window_stack_below(&self)
    {
        self.configure(&[(
            xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW
        )]);
    }

    fn window_draw(&self, surface: &Surface)
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
        self.update();
    }

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

    fn init() -> Self
    {
        // TODO
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

            let (x, y) = (0, 0);
            let (width, height) = (1, 1);
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

            (id, fore)
        };

        Self {
            connection: conn,
            window: id,
            foreground: fore
        }
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

    fn window(&self, command: &WindowCommand)
    {
        use WindowCommand::*;
        match command {
            Title(name) => self.window_title(name),
            Dimension((w, h)) => self.window_resize(*w, *h),
            Origin((x, y)) => self.window_move(*x, *y),
            Map => self.window_map(),
            Unmap => self.window_unmap(),
            StackAbove => self.window_stack_above(),
            StackBelow => self.window_stack_below(),
            Draw(surface) => self.window_draw(surface),
            Update => self.update()
        }
    }

    fn update(&self)
    {
        self.connection.flush();
    }
}

impl Drop for Context {

    fn drop(&mut self)
    {
        self.window_unmap();
    }
}
