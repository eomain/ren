extern crate ren;

use ren::render::Surface;
use ren::render::Drawable;
use ren::render::Font;
use ren::render::Line;
use ren::render::Point;
use ren::render::Rectangle;

#[test]
fn main() {
    let context = match ren::init() {
        Ok(con) => con,
        Err(_) => panic!("ren: cannot initialize!")
    };

    let mut win = ren::Window::new(&context);
    win.set_title("Ren - example");
    win.set_dimension((640, 480));
    win.set_origin((0, 0));

    let points = &[
        Point::new(40, 10),
        Point::new(40, 30),
        Point::new(60, 30),
        Point::new(60, 10),
        Point::new(40, 10)
    ];

    let line = Line::new(points);

    let lines : &[Line] = &[
        line
    ];

    let mut surface = Surface::from(lines);

    ren::map(&mut win);

    ren::events(&win, |event| {
        match event {
            ren::Event::Terminate => {

            },

            ren::Event::Expose(map) => {
                let (x, y) = map.get_position();
                let (width, height) = map.get_dimension();
                println!("pos: ({}, {}), dim: ({}, {})", x, y, width, height);

                ren::draw(&win, &surface);
            },

            ren::Event::Key(event) => {
                match event {
                    ren::KeyEvent::Press(_) => {
                        println!("keypress");
                    },

                    ren::KeyEvent::Release(_) => {
                        println!("keyrelease");
                    }

                    _ => ()
                }
            },

            ren::Event::Mouse(event) => {
                match event {
                    ren::MouseEvent::Press(pos) => {
                        println!("button press: x: {}, y: {}", pos.0, pos.1)
                    },

                    ren::MouseEvent::Release(pos) => {
                        println!("button release: x: {}, y: {}", pos.0, pos.1)
                    },

                    ren::MouseEvent::Hover(pos) => {
                        println!("hover: x: {}, y: {}", pos.0, pos.1)
                    }

                    _ => ()
                }
            }

            _ => ()
        }
    });
}
