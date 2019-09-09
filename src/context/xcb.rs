
use super::DisplayContext;
use display::window::Window;
use display::Manager;
use render;
use render::Surface;
use render::Font;
use render::Point;
use render::Line;
use render::Rectangle as Rect;
use event;
use event::Event;
use event::InputEvent;
use event::DisplayEvent;
use event::input::KeyEvent;
use event::input::MouseEvent;

pub fn init(context: &mut crate::Context)
{
    context.set_map(map);
    context.set_unmap(unmap);
    context.set_draw(draw);
    context.set_event(event);
}

fn map(manager: &Manager)
{
    let display = manager.xcb();
    display.map();
}

fn unmap(manager: &Manager)
{
    let display = manager.xcb();
    display.unmap();
}

fn draw(manager: &Manager, surface: &render::Surface)
{
    let display = manager.xcb();
    display.draw(surface);
    display.refresh();
}

fn event(manager: &Manager) -> Event
{
    let display = manager.xcb();
    display.event()
}

pub struct Context
{
    connection: xcb::Connection,
    window: xcb::Window,
    foreground: u32,
    /*root: xcb::Drawable*/
}

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
        y,
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
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE |
                 xcb::EVENT_MASK_KEY_PRESS |
                 xcb::EVENT_MASK_BUTTON_PRESS |
                 xcb::EVENT_MASK_BUTTON_RELEASE |
                 xcb::EVENT_MASK_POINTER_MOTION |
                 xcb::EVENT_MASK_BUTTON_MOTION)
            ];

            let (x, y) = window.get_origin();
            let (width, height) = window.get_dimension();
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
                window.get_title().as_bytes()
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
                                MouseEvent::Hover(
                                    event::xcb::mouse_hover(&e)
                                )
                            )
                        )
                    },

                    _ => Event::None
                }
            }
        }
    }

    fn draw(&self, surface: &Surface)
    {
        surface.for_each(|object| {
            match *object {
                render::Object::Font(ref f) => {
                    font(self, f);
                },

                render::Object::Point(ref p) => {
                    point(self, p);
                },

                render::Object::Line(ref l) => {
                    line(self, l);
                },

                render::Object::Rect(ref r) => {
                    rect(self, r);
                }
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
