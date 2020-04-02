extern crate ren;
extern crate mirage;

use mirage::convert::svg;
use ren::render::Object::Primitive;
use ren::render::Primitive::Text;
use ren::render::Surface;
use ren::render::Translate;
use ren::render::Font;

fn surface() -> Surface
{
    svg::into::string(r#"
        <svg>
            <text x="280" y="150">hello world</text>
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
                if let ren::DisplayEvent::Expose(_) = event {
                    ren::draw(&win, &surface);
                }
            },

            _ => ()
        }
    });
}
