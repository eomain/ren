extern crate ren;
extern crate mirage;

use mirage::convert::svg;
use ren::render::Surface;
use ren::render::Font;
use ren::render::Line;
use ren::render::Point;
use ren::render::Rect;

fn surface() -> Surface
{
    svg::into::string(r#"
        <svg>
            <line x1="40" y1="10" x2="40" y2="30" />
            <line x1="40" y1="30" x2="60" y2="30" />
            <line x1="60" y1="30" x2="60" y2="10" />
            <line x1="60" y1="10" x2="40" y2="10" />
        </svg>
    "#).unwrap()
}

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

    let surface = surface();

    ren::map(&mut win);

    ren::events(&win, |event| {
        match event {
            ren::Event::Terminate => {

            },

            ren::Event::Display(event) => {
                if let ren::DisplayEvent::Expose(map) = event {
                    let (x, y) = map.position();
                    let (w, h) = map.dimension();
                    println!("pos: ({}, {}), dim: ({}, {})", x, y, w, h);

                    ren::draw(&win, &surface);
                }
            },

            ren::Event::Input(event) => {
                match event {
                    ren::InputEvent::Key(event) => {
                        match event {
                            ren::KeyEvent::Press(_) => {
                                println!("key-press");
                            },

                            ren::KeyEvent::Release(_) => {
                                println!("key-release");
                            }

                            _ => ()
                        }
                    },

                    ren::InputEvent::Mouse(event) => {
                        match event {
                            ren::MouseEvent::Press(pos) => {
                                println!("button-press: x: {}, y: {}", pos.0, pos.1)
                            },

                            ren::MouseEvent::Release(pos) => {
                                println!("button-release: x: {}, y: {}", pos.0, pos.1)
                            },

                            ren::MouseEvent::Hover(pos) => {
                                println!("hover: x: {}, y: {}", pos.0, pos.1)
                            }

                            _ => ()
                        }
                    },

                    _ => ()
                }
            },

            _ => ()
        }
    });
}
