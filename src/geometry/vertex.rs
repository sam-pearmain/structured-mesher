use std::marker::PhantomData;

use super::prelude::*;
use super::points::{Dimensioned, Point, Point2D, Point3D};

pub struct Vertex<P: Point> {
    id: usize,
    coords: P,
}

impl<P: Point> Dimensioned for Vertex<P> {
    fn is_2d(&self) -> bool {
        self.coords.is_2d()
    }

    fn dimensions(&self) -> usize {
        if self.is_2d() { 2 } else { 3 }
    }
}

pub struct VertexBuilder<F: Float> {
    phantom: PhantomData<F>,
}

impl<F: Float> VertexBuilder<F> {
    pub fn new_2d(id: usize, x: F, y: F) -> Vertex<Point2D<F>> {
        Vertex { id: id, coords: Point2D::new(x, y) }
    }

    pub fn new_3d(id: usize, x: F, y: F, z: F) -> Vertex<Point3D<F>> {
        Vertex { id: id, coords: Point3D::new(x, y, z) }
    }
}