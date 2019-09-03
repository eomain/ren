extern crate ren;

use ren::render::Surface;
use ren::render::Font;
use ren::render::Point;
use ren::render::Rectangle;


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

    let point = Point::new(40, 10);
    let mut rect = Rectangle::from_point(&point, 10, 10);
    rect.translate((160, 4));
    rect.scale(16.0);

    let rects = [
        rect,
        Rectangle::new(200, 200, 160, 160),
        Rectangle::new(400, 100, 30, 30),
        Rectangle::new(400, 250, 80, 80)
    ];

    let surface = Surface::from(&rects);

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
