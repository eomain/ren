extern crate ren;
extern crate mirage;

use mirage::convert::svg;
use ren::render::Object;
use ren::render::Surface;
use ren::render::Font;
use ren::render::Point;
use ren::render::Rect;

fn surface() -> Surface
{
    svg::into::string(r#"
        <svg>
            <line x1="0" y1="0" x2="640" y2="480" />
            <rect width="639" height="479" />
            <line x1="640" y1="0" x2="0" y2="480" />
        </svg>
    "#).unwrap()
}

fn main()
{
    let context = match ren::init() {
        Ok(con) => con,
        Err(_) => panic!("ren: cannot initialize!")
    };

    let title = format!("Ren - example {}", file!());

    let mut win = ren::Window::new(&context);
    win.set_title(&title);
    win.set_dimension((640, 480));
    win.set_origin((0, 0));

    let surface = surface();

    ren::map(&mut win);

    ren::events(&win, |event| {
        match event {
            ren::Event::Terminate => {

            },

            ren::Event::Display(event) => {
                match event {
                    ren::DisplayEvent::Expose(map) => {
                        ren::draw(&win, &surface);
                    },

                    _ => ()
                }
            },

            _ => ()
        }
    });
}
