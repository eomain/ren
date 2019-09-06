
pub(crate) mod xcb;

#[derive(Clone)]
pub enum Object<'a> {
    Font(Font<'a>),
    Point(Point),
    Line(Line),
    Rect(Rectangle)
}

pub trait Drawable: Clone {

    fn draw(&self);

    fn object(&self) -> Object;

}

pub struct Surface<'a> {
    objects: Vec<Object<'a>>
}

impl<'a> Surface<'a> {

    pub fn new() -> Self
    {
        Self {
            objects: Vec::new()
        }
    }

    pub fn from<T>(objects: &'a [T]) -> Self
        where T: Drawable
    {
        Self {
            objects: objects.iter().map(|o| o.object()).collect()
        }
    }

    pub fn from_object(objects: &[Object<'a>]) -> Self
    {
        Self {
            objects: objects.to_vec()
        }
    }
}

impl<'a> Surface<'a> {

    pub fn add<T>(&mut self, objects: &'a [T])
        where T: Drawable
    {
        let mut v = objects.iter().map(|o| o.object()).collect();
        self.objects.append(&mut v);
    }

    pub fn add_object(&mut self, objects: &[Object<'a>])
    {
        let mut v = objects.to_vec();
        self.objects.append(&mut v);
    }

    pub fn for_each<F>(&self, func: F)
        where F: Fn(&Object)
    {
        self.objects.iter().for_each(func);
    }

}

#[derive(Clone)]
pub enum Color {
    RGB(u16, u16, u16)
}

#[derive(Clone)]
pub struct Font<'a> {
    text: &'a str,
    font: Option<&'a str>,
    color: Color,
    point: Point
}

impl<'a> Font<'a> {

    pub fn new(text: &'a str) -> Self
    {
        Self {
            text,
            font: None,
            color: Color::RGB(0, 0, 0),
            point: Point::new(0, 0)
        }
    }
}

impl<'a> Font<'a> {

    pub fn get_text(&self) -> &str
    {
        &self.text
    }

    pub fn get_point(&self) -> &Point
    {
        &self.point
    }

    pub fn set_position(&mut self, pos: (i32, i32))
    {
        self.point.set(pos);
    }
}

impl<'a> Drawable for Font<'a> {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Font(self.clone())
    }
}

pub struct Image {
    buffer: Vec<u8>,
    point: Point,
    width: u32,
    height: u32
}

#[derive(Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {

    pub fn new(x: i32, y: i32) -> Self
    {
        Self {
            x, y
        }
    }
}

impl Point {

    pub fn set(&mut self, point: (i32, i32))
    {
        self.x = point.0;
        self.y = point.1;
    }

    pub fn set_x(&mut self, x: i32)
    {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32)
    {
        self.y = y;
    }

    pub fn translate_x(&mut self, x: i32)
    {
        self.x += x;
    }

    pub fn translate_y(&mut self, y: i32)
    {
        self.y += y;
    }
}

impl From<(i32, i32)> for Point {

    fn from(point: (i32, i32)) -> Self
    {
        Self::new(point.0, point.1)
    }

}

impl From<&Point> for (i32, i32) {

    fn from(point: &Point) -> Self
    {
        (point.x, point.y)
    }
}

impl Drawable for Point {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Point(self.clone())
    }

}

#[derive(Clone)]
pub struct Line {
    points: Vec<Point>
}

impl Line {

    pub fn new(points: &[Point]) -> Self
    {
        Self {
            points: points.to_vec()
        }
    }
}

impl Line {

    pub fn points(&self) -> &[Point]
    {
        &self.points
    }

    pub fn for_each<F>(&self, mut func: F)
        where F: Fn(&Point)
    {
        self.points.iter().for_each(func);
    }
}

impl Drawable for Line {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Line(self.clone())
    }

}

#[derive(Clone)]
pub struct Rectangle {
    point: Point,
    width: u32,
    height: u32
}

impl Rectangle {

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self
    {
        Self {
            point: Point::new(x, y),
            width,
            height
        }
    }

    pub fn from_point(point: &Point, width: u32, height: u32) -> Self
    {
        let (x, y) = point.into();
        Self::new(x, y, width, height)
    }
}

impl Rectangle {

    pub fn get_point(&self) -> &Point
    {
        &self.point
    }

    pub fn get_width(&self) -> u32
    {
        self.width
    }

    pub fn get_height(&self) -> u32
    {
        self.height
    }

    pub fn set_width(&mut self, width: u32)
    {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32)
    {
        self.height = height;
    }


    pub fn set_dimension(&mut self, dimension: (u32, u32))
    {
        self.width = dimension.0;
        self.height = dimension.1;
    }

    pub fn translate(&mut self, location: (i32, i32))
    {
        if location.0 != 0 {
            self.point.translate_x(location.0);
        }

        if location.1 != 0 {
            self.point.translate_y(location.1);
        }
    }

    pub fn scale(&mut self, factor: f64)
    {
        let w = (self.get_width() as f64 * factor);
        let h = (self.get_height() as f64 * factor);
        self.set_dimension((w as u32, h as u32));
    }

}

impl<'a> From<&'a Rectangle> for (&'a Point, u32, u32)
{
    fn from(rect: &'a Rectangle) -> Self
    {
        (&rect.point, rect.width, rect.height)
    }
}

impl Drawable for Rectangle {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Rect(self.clone())
    }

}
