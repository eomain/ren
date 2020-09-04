
extern crate xcb;

use super::DisplayContext;
use crate::{
    Stat,
    Data,
    XcbStat,
    XcbData,
    WindowCommand,
    Event,
    DisplayEvent,
    KeyEvent,
    MouseEvent,
    display::Manager,
    render,
    render::{
        Surface
    },
    event
};

pub struct Context {
    pub connection: xcb::Connection,
    pub window: xcb::Window,
    pub foreground: u32
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
                        super::render::font(self, f);
                    },

                    Primitive::Point(ref p) => {
                        super::render::point(self, p);
                    },

                    Primitive::Line(ref l) => {
                        super::render::line(self, l);
                    },

                    Primitive::Rect(ref r) => {
                        super::render::rect(self, r);
                    }
                },
                _ => ()
            }
        });
        self.update();
    }

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
                        DisplayEvent::Expose(event::xcb::expose(&e)).into()
                    },

                    xcb::KEY_PRESS => {
                        KeyEvent::Press(event::xcb::key_press(&e)).into()
                    },

                    xcb::KEY_RELEASE => {
                        KeyEvent::Release(event::xcb::key_release(&e)).into()
                    },

                    xcb::BUTTON_PRESS => {
                        MouseEvent::Press(event::xcb::button_press(&e)).into()
                    },

                    xcb::BUTTON_RELEASE => {
                        MouseEvent::Release(event::xcb::button_release(&e)).into()
                    },

                    xcb::MOTION_NOTIFY => {
                        MouseEvent::Move(event::xcb::mouse_move(&e)).into()
                    },

                    xcb::ENTER_NOTIFY => {
                        MouseEvent::Enter(event::xcb::mouse_enter(&e)).into()
                    },

                    xcb::LEAVE_NOTIFY => {
                        MouseEvent::Leave(event::xcb::mouse_leave(&e)).into()
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
