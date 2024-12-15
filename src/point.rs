use std::ops::Add;

type Coord = i32;

pub const NORTH: Point = Point::new(0, -1);
pub const EAST: Point = Point::new(1, 0);
pub const SOUTH: Point = Point::new(0, 1);
pub const WEST: Point = Point::new(-1, 0);

pub const ORTHO_DIR: [Point; 4] = [NORTH, SOUTH, WEST, EAST];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}
impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: Coord, y: Coord) -> Self {
        Point { x, y }
    }
}
