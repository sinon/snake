use std::ops::IndexMut;
use std::{fmt::Debug, ops::Index};

use crate::point::Point;

#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    pub fn pos(&self, p: usize) -> Point {
        Point::new((p % self.width) as i32, (p / self.width) as i32)
    }
    fn idx(&self, p: &Point) -> usize {
        ((self.width as i32) * p.y + p.x) as usize
    }

    pub fn try_get<U: AsRef<Point>>(&self, p: U) -> Option<&T> {
        if self.contains(p.as_ref()) {
            Some(&self[*p.as_ref()])
        } else {
            None
        }
    }

    pub fn try_get_mut<U: AsRef<Point>>(&mut self, p: U) -> Option<&mut T> {
        if self.contains(p.as_ref()) {
            Some(&mut self[*p.as_ref()])
        } else {
            None
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Point) -> &Self::Output {
        &self.cells[self.idx(&pos)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, pos: Point) -> &mut Self::Output {
        let idx = self.idx(&pos);
        &mut self.cells[idx]
    }
}

impl<T: Debug> Grid<T> {
    pub fn print(&self) {
        println!("Grid {w}x{h}", w = &self.width, h = &self.height);
        for row in 0..self.height {
            println!(
                "r{row}: {:?}",
                &self.cells[row * self.width..(row + 1) * self.width]
            );
        }
    }
}
