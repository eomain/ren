
pub(crate) mod xcb;

#[derive(Clone)]
pub enum Object<'a> {
    Font(Font<'a>),
    Point(Point),
    Line(Line),
    Rect(Rectangle)
}

pub trait Drawable: Clone {

    fn draw(&self)
    {

    }

    fn object(&self) -> Object;

}

pub trait Transformation {

    fn point(&self) -> &Point;

    fn points(&self) -> Option<&[Point]>
    {
        None
    }

    fn point_mut(&mut self) -> &mut Point;

    fn points_mut(&mut self) -> Option<&mut [Point]>
    {
        None
    }

    fn for_each<F>(&mut self, pos: (i32, i32), mut func: F)
        where F: FnMut(&mut Point, (i32, i32))
    {
        if match self.points_mut() {
            None => true,
            Some(points) => {
                points.iter_mut().for_each(|p| func(p, pos));
                false
            }
        } {
            func(self.point_mut(), pos);
        }
    }

    fn position(&mut self, pos: (i32, i32))
    {
        self.for_each(pos, |point, pos| {
            point.x = pos.0;
            point.y = pos.1;
        });
    }

    fn translate(&mut self, pos: (i32, i32))
    {
        self.for_each(pos, |point, pos| {
            point.x += pos.0;
            point.y += pos.1;
        });
    }

}

#[test]
fn translation()
{
    let mut point: Point = (3, 4).into();
    point.translate((3, 3));
    let pos: (_, _) = point.into();
    assert_eq!(pos, (6, 7));
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

impl<'a, T> From<&'a [T]> for Surface<'a>
    where T: Drawable
{
    fn from(objects: &'a [T]) -> Self
    {
        Self {
            objects: objects.iter().map(|o| o.object()).collect()
        }
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
}

impl<'a> Transformation for Font<'a> {

    fn point(&self) -> &Point
    {
        &self.point
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.point
    }

}

impl<'a> Drawable for Font<'a> {

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

#[derive(Clone, Debug)]
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

impl From<Point> for (i32, i32) {

    fn from(point: Point) -> Self
    {
        (point.x, point.y)
    }
}

impl From<&Point> for (i32, i32) {

    fn from(point: &Point) -> Self
    {
        (point.x, point.y)
    }
}

impl Transformation for Point {

    fn point(&self) -> &Point
    {
        self
    }

    fn point_mut(&mut self) -> &mut Point
    {
        self
    }

}

impl Drawable for Point {

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

    pub fn new() -> Self
    {
        Self {
            points: Vec::new()
        }
    }

    pub fn from(points: &[Point]) -> Self
    {
        Self {
            points: points.to_vec()
        }
    }
}

impl Line {

    pub fn append(&mut self, point: Point)
    {
        self.points.push(point);
    }

    pub fn count(&self) -> usize
    {
        self.points.len()
    }

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

impl From<&[Point]> for Line {

    fn from(points: &[Point]) -> Self
    {
        Self {
            points: points.to_vec()
        }
    }

}

impl Transformation for Line {

    fn point(&self) -> &Point
    {
        &self.points[0]
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.points[0]
    }

}

impl Drawable for Line {

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

    pub fn new() -> Self
    {
        Self::from(0, 0, 0, 0)
    }

    pub fn from(x: i32, y: i32, width: u32, height: u32) -> Self
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
        Self::from(x, y, width, height)
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

impl Transformation for Rectangle {

    fn point(&self) -> &Point
    {
        &self.point
    }

    fn point_mut(&mut self) -> &mut Point
    {
        &mut self.point
    }

}

impl Drawable for Rectangle {

    fn object(&self) -> Object
    {
        Object::Rect(self.clone())
    }

}
