
pub(crate) mod xcb;

#[derive(Clone)]
pub enum Object<'a> {
    Font(&'a Font<'a>),
    Point(&'a Point),
    Line(&'a Line<'a>),
    Rect(&'a Rectangle)
}

pub trait Drawable {

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
        let mut surface = Self::new();
        surface.add(objects);
        surface
    }

    pub fn from_object(objects: &[Object<'a>]) -> Self
    {
        let mut surface = Self::new();
        surface.add_object(objects);
        surface
    }
}

impl<'a> Surface<'a> {

    pub fn add<T>(&mut self, objects: &'a [T])
        where T: Drawable
    {
        let iter = objects.iter();
        self.objects.reserve(iter.len());

        for o in iter {
            self.objects.push(o.object());
        }
    }

    pub fn add_object(&mut self, objects: &[Object<'a>])
    {
        let iter = objects.iter();
        self.objects.reserve(iter.len());

        for o in iter {
            self.objects.push(o.clone());
        }
    }

    pub fn for_each<F>(&self, func: F)
        where F: Fn(&Object)
    {
        for obj in &self.objects {
            func(obj);
        }
    }

}

pub enum Color {
    RGB(u16, u16, u16)
}

pub struct Font<'a> {
    text: &'a str,
    font: String,
    color: Color,
    point: Point
}

impl<'a> Font<'a> {

    pub fn new(text: &'a str) -> Self
    {
        Self {
            text,
            font: String::new(),
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
        Object::Font(self)
    }
}

pub struct Image {
    buffer: Vec<u8>,
    point: Point,
    width: u32,
    height: u32
}

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

    pub fn get(&self) -> (i32, i32)
    {
        (self.x, self.y)
    }

    pub fn get_x(&self) -> i32
    {
        self.x
    }

    pub fn get_y(&self) -> i32
    {
        self.y
    }

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

impl Drawable for Point {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Point(self)
    }

}

pub struct Line<'a> {
    points: &'a [Point]
}

impl<'a> Line<'a> {

    pub fn new(points: &'a [Point]) -> Self
    {
        Self {
            points
        }
    }
}

impl<'a> Line<'a> {

    pub fn get(&self) -> &'a [Point]
    {
        &self.points
    }

    pub fn for_each<F>(&self, mut func: F)
        where F: Fn(&Point)
    {
        for point in self.points {
            func(point);
        }
    }
}

impl<'a> Drawable for Line<'a> {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Line(self)
    }

}

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
        Self::new(point.get_x(), point.get_y(), width, height)
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
        let (w, h) = {
            ((self.get_width() as f64 * factor) as u32,
             (self.get_height() as f64 * factor) as u32)
        };
        self.set_dimension((w, h));
    }

}

impl Drawable for Rectangle {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object
    {
        Object::Rect(self)
    }

}
