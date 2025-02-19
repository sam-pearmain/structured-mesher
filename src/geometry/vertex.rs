#![allow(dead_code)]

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

impl<F: Float> Vertex<Point2D<F>> {
    pub fn new_2d(id: usize, x: F, y: F) -> Vertex<Point2D<F>> {
        Vertex { id: id, coords: Point2D::new(x, y) }
    }
}

impl<F: Float> Vertex<Point3D<F>> {
    pub fn new_3d(id: usize, x: F, y: F, z: F) -> Vertex<Point3D<F>> {
        Vertex { id: id, coords: Point3D::new(x, y, z) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_2d_creation() {
        let vertex = Vertex::new_2d(1, 1f32, 1f32);
        assert_eq!(vertex.id, 1);
        assert_eq!(vertex.coords, Point2D::new(1f32, 1f32));
        assert_eq!(vertex.dimensions(), 2);
        assert!(vertex.is_2d());
    }

    #[test]
    fn test_vertex_3d_creation() {
        let vertex = Vertex::new_3d(2, 2f64, 3f64, 4f64);
        assert_eq!(vertex.id, 2);
        assert_eq!(vertex.coords, Point3D::new(2f64, 3f64, 4f64));
        assert_eq!(vertex.dimensions(), 3);
        assert!(!vertex.is_2d());
    }
}