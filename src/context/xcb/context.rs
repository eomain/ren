
extern crate xcb;

use crate::{
    Stat, Data, WindowCommand, Event, DisplayEvent, KeyEvent,
    MouseEvent, event, render, render::{Image, Surface}
};

pub struct Context {
    pub connection: xcb::Connection,
    pub window: xcb::Window,
    pub black: u32,
    pub white: u32,
    visual: Option<xcb::Visualtype>,
    delete: Option<xcb::Atom>
}

impl Context {

    pub fn id(&self) -> u32
    {
        self.connection.generate_id()
    }

    fn property<T>(&self, mode: xcb::PropMode, prop: xcb::AtomEnum, ty: xcb::AtomEnum, data: &[T])
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

    fn geometry(&self) -> Option<xcb::GetGeometryReply>
    {
        xcb::get_geometry(&self.connection, self.window).get_reply().ok()
    }

    fn stat_position(&self) -> Option<(u32, u32)>
    {
        self.geometry().map(|g| (g.x() as u32, g.y() as u32))
    }

    fn stat_dimension(&self) -> Option<(u32, u32)>
    {
        self.geometry().map(|g| (g.width() as u32, g.height() as u32))
    }

    fn stat_depth(&self) -> Option<u8>
    {
        self.geometry().map(|g| g.depth())
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
    }

    fn window_image(&self, img: &Image)
    {
        let (x, y) = (img.point.x as i16, img.point.y as i16);
        let (w, h) = (img.width as u16, img.height as u16);
        let image = super::render::Image::from_window(self, w, h);
        image.write(self, 0, 0, w, h, &img.data);
        image.draw_window(self, (0, 0), (x, y), w, h);
    }

    fn window_event(&self, event: Option<xcb::base::GenericEvent>) -> Option<Event>
    {
        if event.is_none() {
            if let Err(_) = self.connection.has_error() {
                return Some(Event::Terminate);
            }
        }

        event.map(|e| {
            let response = event_type(&e);

            match response {
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

                xcb::FOCUS_IN => DisplayEvent::FocusIn.into(),

                xcb::FOCUS_OUT => DisplayEvent::FocusOut.into(),

                xcb::CLIENT_MESSAGE => {
                    let event = unsafe { xcb::cast_event::<xcb::ClientMessageEvent>(&e) };
                    if event.type_() == 32 {
                        if let Some(del) = self.delete {
                            if del == event.data().data32()[0] {
                                return Event::Terminate;
                            }
                        }
                    }
                    Event::Unknown(Some(response.into()))
                },

                _ => Event::Unknown(Some(response.into()))
            }
        })
    }

    fn window_clear(&self)
    {
        let g = match self.geometry() {
            None => return,
            Some(g) => g
        };
        let x = g.x();
        let y = g.y();
        let w = g.width();
        let h = g.height();
        xcb::clear_area(&self.connection, false, self.window, x, y, h, w);
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
    xcb::EVENT_MASK_LEAVE_WINDOW |
    xcb::EVENT_MASK_FOCUS_CHANGE
    /*xcb::EVENT_MASK_RESIZE_REDIRECT |
    xcb::EVENT_MASK_STRUCTURE_NOTIFY |
    xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY |
    xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT |
    xcb::EVENT_MASK_VISIBILITY_CHANGE*/
);

#[inline]
fn event_type(e: &xcb::base::GenericEvent) -> u8
{
    e.response_type() & !0x080
}

fn window(conn: &xcb::Connection, screen: &xcb::Screen) -> u32
{
    let id = conn.generate_id();

    let values = [
        (xcb::CW_EVENT_MASK, EVENT_MASK)
    ];

    let (x, y) = (0, 0);
    let (width, height) = (1, 1);
    let border = 10;

    xcb::create_window(
        conn,
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
    id
}

#[inline]
fn gc(conn: &xcb::Connection, window: xcb::Window, color: u32) -> u32
{
    let id = conn.generate_id();
    xcb::create_gc(
        conn, id, window, &[
            (xcb::GC_FOREGROUND,
             color),
            (xcb::GC_GRAPHICS_EXPOSURES,
             0)
        ]
    );
    id
}

impl super::SystemContext for Context {

    fn init() -> Self
    {
        // TODO
        let (connect, num) = xcb::Connection::connect(None).unwrap();
        let setup = connect.get_setup();
        let screen = setup.roots().nth(num as usize).unwrap();
        let depths = screen.allowed_depths();
        let mut depth = None;
        for d in depths {
            if d.depth() == 24 {
                depth = Some(d);
            }
        }
        let visuals = depth.map(|d| d.visuals());
        let mut visual = None;
        if let Some(visuals) = visuals {
            for v in visuals {
                if v.class() as u32 == xcb::VISUAL_CLASS_TRUE_COLOR {
                    visual = Some(v);
                    break;
                }
            }
        }
        let window = window(&connect, &screen);
        let root = screen.root();
        let black = gc(&connect, root, screen.black_pixel());
        let white = gc(&connect, root, screen.white_pixel());

        let cookie = xcb::intern_atom(&connect, true, "WM_PROTOCOLS");
        cookie.get_reply();
        let cookie = xcb::intern_atom(&connect, false, "WM_DELETE_WINDOW");
        let delete = cookie.get_reply().ok().map(|r| r.atom());

        Self {
            connection: connect,
            window,
            black,
            white,
            visual,
            delete
        }
    }

    fn event(&self) -> Option<Event>
    {
        let event = self.connection.wait_for_event();
        self.window_event(event)
    }

    fn poll(&self) -> Option<Event>
    {
        let event = self.connection.poll_for_event();
        self.window_event(event)
    }

    fn stat(&self, status: Stat) -> Option<Data>
    {
        use crate::{stat::{WindowStat, XcbStat}, data::{WindowData, XcbData}};

        match status {
            Stat::Window(status) => {
                Some((match status {
                    WindowStat::Position => WindowData::Position(self.stat_position()?),
                    WindowStat::Dimension => WindowData::Dimension(self.stat_dimension()?),
                    WindowStat::Depth => WindowData::Depth(self.stat_depth()?)
                }).into())
            },
            Stat::Xcb(status) => {
                Some((match status {
                    XcbStat::Connection => XcbData::Connection(self.connection.get_raw_conn()),
                    XcbStat::Window => XcbData::Window(self.window),
                    XcbStat::VisualType => XcbData::VisualType(self.visual)
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
            Image(image) => self.window_image(image),
            Clear => self.window_clear(),
            Update => self.update()
        }
    }

    fn update(&self)
    {
        self.connection.flush();
    }
}
