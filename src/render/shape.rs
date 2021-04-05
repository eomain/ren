//! Vector graphics type primitives

use std::ops::Add;

/// A Point is a simple object that
/// represents a single location
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Point {
    /// The x coordinate
    pub x: isize,
    /// The y coordinate
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self
    {
        Self {
            x, y
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output
    {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from(p: (isize, isize)) -> Self
    {
        Self::new(p.0, p.1)
    }
}

impl From<&Point> for (isize, isize) {
    fn from(p: &Point) -> Self
    {
        (p.x, p.y)
    }
}

/// A rectangular area with an origin (Point),
/// as well as a width and height
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Rect {
    /// The origin of the rectangle
    pub point: Point,
    /// The width of the rectangle
    pub width: usize,
    /// The height of the rectangle
    pub height: usize
}

impl Rect {
    pub fn new<P>(p: P, width: usize, height: usize) -> Self
        where P: Into<Point>
    {
        Self {
            point: p.into(),
            width,
            height
        }
    }
}

impl<'a> From<&'a Rect> for (&'a Point, usize, usize)
{
    fn from(rect: &'a Rect) -> Self
    {
        (&rect.point, rect.width, rect.height)
    }
}
