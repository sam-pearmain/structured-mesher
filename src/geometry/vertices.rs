#![allow(dead_code)]

use std::marker::PhantomData;

use super::points::{Dimensioned, Point, Dimensions, Point2D, Point3D};
use super::vertex::Vertex;

/// should be able to store both 2d and 3d sets of vertices and 
/// have common functoinality to interpret the vertices
pub struct VertexCollection<P: Point> {
    vertices: Vec<Vertex<P>>,
    dimensions: Dimensions,
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

    fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    fn total_vertices(&self) -> usize {
        self.vertices.len()
    }
}

pub struct VertexCollectionBuilder<P: Point> {
    dimensions: Option<Dimensions>,
    _phantom: PhantomData<P>,
}

impl<P: Point> VertexCollectionBuilder<P> {
    pub fn build(self) -> Result<VertexCollection<P>, &'static str> {
        let dimensions = match self.dimensions {
            Some(d) => d,
            None => return Err("dimensions must be set before attempting to build a vertex collection"),
        };
        
        Ok(VertexCollection {
            vertices: Vec::new(),
            dimensions
        })
    }
}

impl VertexCollectionBuilder<Point2D> {
    pub fn new_2d(nx: usize, ny: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Some(Dimensions::Two { nx, ny }),
            _phantom: PhantomData,
        }
    }
}

impl VertexCollectionBuilder<Point3D> {
    pub fn new_3d(nx: usize, ny: usize, nz: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Some(Dimensions::Three { nx, ny, nz }),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_collection_builder() {
        let collection = VertexCollectionBuilder::new_2d(10, 10)
            .build();
        assert!(collection.is_ok());
    }
}