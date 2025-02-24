#![allow(dead_code)]

use num_traits::Float;
use super::{points::{Dimensioned, Point, Point2D, Point3D}, vertex::Vertex};

pub struct Line<'a, P: Point> {
    pub start: &'a Vertex<P>,
    pub end:   &'a Vertex<P>,
}

impl<'a, F: Float> Line<'a, Point2D<F>> {
    pub fn new_2d(v1: &'a Vertex<Point2D<F>>, v2: &'a Vertex<Point2D<F>>) -> Self {
        Line { start: v1, end: v2 }
    }
}

impl<'a, F: Float> Line<'a, Point3D<F>> {
    pub fn new_3d(v1: &'a Vertex<Point3D<F>>, v2: &'a Vertex<Point3D<F>>) -> Self {
        Line { start: v1, end: v2 }
    }
}

impl<'a, P: Point> Dimensioned for Line<'a, P> {
    fn is_2d(&self) -> bool {
        if self.dimensions() == 2 { true } else { false }
    }
    
    fn dimensions(&self) -> usize {
        self.start.dimensions()
    }
}

pub struct LineCollection<'a, P: Point> {
    lines: Vec<Line<'a, P>>,
}

impl<'a, P: Point> Dimensioned for LineCollection<'a, P> {
    fn is_2d(&self) -> bool {
        let first = self.lines.first().unwrap();
        first.is_2d()
    }

    fn dimensions(&self) -> usize {
        let first = self.lines.first().unwrap();
        first.dimensions()
    }
}

impl<'a, P: Point> LineCollection<'a, P> {
    pub fn new() -> Self {
        LineCollection { lines: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn add_line(&mut self, l: Line<'a, P>) -> Result<(), &'static str>{
        if l.dimensions() == self.dimensions() {
            self.lines.push(l);
        } else {
            return Err("can't add line of differing dimensions to what was already stored in the line collection");
        }
        Ok(())
    }
}