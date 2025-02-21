#![allow(dead_code)]

use std::marker::PhantomData;
use super::points::{Dimensioned, Point2D, Point3D};
use super::{points::{Dimensions, Point}, vertex::Vertex};

pub enum WriteOrder {
    IJK, // write row, then column, then layer
    JIK, // write column, then row, then layer
}

/// should be able to store both 2d and 3d sets of vertices and 
/// have common functoinality to interpret the vertices
pub struct VertexCollection<P: Point> {
    vertices: Vec<Vertex<P>>,
    dimensions: Dimensions,
    write_order: WriteOrder,
}

impl<P: Point> Dimensioned for VertexCollection<P> {
    fn is_2d(&self) -> bool {
        self.dimensions.is_2d()
    }

    fn dimensions(&self) -> usize {
        self.dimensions.dimensions()
    }
}

impl<P: Point> VertexCollection<P> {
    fn add_vertex(&mut self, v: Vertex<P>) {
        if self.dimensions() == v.dimensions() {
            self.vertices.push(v);
        }
    }

    fn export_csv(&self, filename: &str) {
        
    }
}

pub struct VertexCollectionBuilder<P: Point> {
    dimensions: Dimensions,
    write_order: Option<WriteOrder>,
    _phantom: PhantomData<P>,
}

impl<P: Point> VertexCollectionBuilder<P> {
    pub fn set_write_order(mut self, write_order: WriteOrder) -> Self {
        self.write_order = Some(write_order);
        self
    }

    pub fn build(self) -> Result<VertexCollection<P>, &'static str> {
        if self.write_order.is_none() {
            return Err("write order not set, cannot build")
        }
        Ok(VertexCollection {
            vertices: Vec::new(),
            dimensions: self.dimensions,
            write_order: self.write_order.unwrap(),
        })
    }
}

impl VertexCollectionBuilder<Point2D<f32>> {
    pub fn new_2d_f32(nx: usize, ny: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Dimensions::Two { nx, ny },
            write_order: None,
            _phantom: PhantomData,
        }
    }
}

impl VertexCollectionBuilder<Point2D<f64>> {
    pub fn new_2d_f64(nx: usize, ny: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Dimensions::Two { nx, ny },
            write_order: None,
            _phantom: PhantomData,
        }
    }
}

impl VertexCollectionBuilder<Point3D<f32>> {
    pub fn new_3d_f32(nx: usize, ny: usize, nz: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Dimensions::Three { nx, ny, nz },
            write_order: None, 
            _phantom: PhantomData,
        }
    }
}

impl VertexCollectionBuilder<Point3D<f64>> {
    pub fn new_3d_f64(nx: usize, ny: usize, nz: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Dimensions::Three { nx, ny, nz },
            write_order: None, 
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_collection_builder() {
        let collection_2d = VertexCollectionBuilder::new_2d_f32(9, 9)
            .set_write_order(WriteOrder::IJK)
            .build();
        assert!(collection_2d.is_ok());
    }

    #[test]
    fn test_2d_f32_builder_success() {
        let vc = VertexCollectionBuilder::new_2d_f32(10, 20)
            .set_write_order(WriteOrder::IJK)
            .build();
        assert!(vc.is_ok(), "2D f32 builder should build successfully");
        let collection = vc.unwrap();
        assert_eq!(collection.dimensions.dimensions(), 2);
        assert!(collection.vertices.is_empty());
    }

    #[test]
    fn test_2d_f64_builder_success() {
        let vc = VertexCollectionBuilder::new_2d_f64(15, 25)
            .set_write_order(WriteOrder::JIK)
            .build();
        assert!(vc.is_ok(), "2D f64 builder should build successfully");
        let collection = vc.unwrap();
        assert_eq!(collection.dimensions.dimensions(), 2);
        assert!(collection.vertices.is_empty());
    }

    #[test]
    fn test_3d_f32_builder_success() {
        let vc = VertexCollectionBuilder::new_3d_f32(8, 16, 32)
            .set_write_order(WriteOrder::IJK)
            .build();
        assert!(vc.is_ok(), "3D f32 builder should build successfully");
        let collection = vc.unwrap();
        assert_eq!(collection.dimensions.dimensions(), 3);
        assert!(collection.vertices.is_empty());
    }

    #[test]
    fn test_3d_f64_builder_success() {
        let vc = VertexCollectionBuilder::new_3d_f64(12, 24, 36)
            .set_write_order(WriteOrder::JIK)
            .build();
        assert!(vc.is_ok(), "3D f64 builder should build successfully");
        let collection = vc.unwrap();
        assert_eq!(collection.dimensions.dimensions(), 3);
        assert!(collection.vertices.is_empty());
    }

    #[test]
    fn test_builder_missing_write_order() {
        let vc = VertexCollectionBuilder::new_2d_f32(10, 20)
            .build();
        assert!(vc.is_err(), "Builder with no write order should error");
    }

    #[test]
    fn test_add_vertex_to_collection() {
        let mut collection = VertexCollectionBuilder::new_2d_f32(10, 20)
            .set_write_order(WriteOrder::IJK)
            .build()
            .expect("Builder should succeed");

        let vertex = Vertex::new_2d(1, 10_f32, 10_f32);
        collection.add_vertex(vertex);

        assert_eq!(collection.vertices.len(), 1);
    }
}