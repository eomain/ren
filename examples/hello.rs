extern crate ren;

use ren::render::Surface;
use ren::render::Font;

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

    let mut f = Font::new("hello world");
    f.set_position((280, 150));
    let font = &[ f ];

    let surface = Surface::from(font);

    ren::map(&mut win);

    ren::events(&win, |event| {
        match event {
            ren::Event::Terminate => {

            },

            ren::Event::Expose(map) => {
                ren::draw(&win, &surface);
            },

            _ => ()
        }
    });
}
