#![allow(dead_code)]

use super::points::{Dimensioned, Point, Point2D, Point3D};

pub struct Vertex<P: Point> {
    id: usize,
    coords: P,
}

impl<P: Point> Vertex<P> {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_x(&self) -> f64 {
        self.coords.x()
    }

    pub fn get_y(&self) -> f64 {
        self.coords.y()
    }

    pub fn get_z(&self) -> f64 {
        self.coords.z()
    }
}

impl<P: Point> Dimensioned for Vertex<P> {
    fn is_2d(&self) -> bool {
        self.coords.is_2d()
    }

    fn dimensions(&self) -> usize {
        if self.is_2d() { 2 } else { 3 }
    }
}

impl Vertex<Point2D> {
    pub fn new_2d(id: usize, x: f64, y: f64) -> Vertex<Point2D> {
        Vertex { id: id, coords: Point2D::new(x, y) }
    }
}

impl Vertex<Point3D> {
    pub fn new_3d(id: usize, x: f64, y: f64, z: f64) -> Vertex<Point3D> {
        Vertex { id: id, coords: Point3D::new(x, y, z) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_2d_creation() {
        let vertex = Vertex::new_2d(1, 1.0, 1.0);
        assert_eq!(vertex.id, 1);
        assert_eq!(vertex.coords, Point2D::new(1.0, 1.0));
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