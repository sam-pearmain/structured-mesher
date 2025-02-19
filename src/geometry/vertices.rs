use std::marker::PhantomData;

use num_traits::Float;

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

impl<F: Float> VertexCollectionBuilder<Point2D<F>> {
    pub fn new_2d(nx: usize, ny: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Dimensions::Two { nx, ny },
            write_order: None,
            _phantom: PhantomData,
        }
    }

    pub fn set_f32(self) -> VertexCollectionBuilder<Point2D<f32>> {
        VertexCollectionBuilder {
            dimensions: self.dimensions,
            write_order: self.write_order,
            _phantom: PhantomData,
        }
    }

    pub fn set_f64(self) -> VertexCollectionBuilder<Point2D<f64>> {
        VertexCollectionBuilder {
            dimensions: self.dimensions,
            write_order: self.write_order,
            _phantom: PhantomData,
        }
    }
}

impl<F: Float> VertexCollectionBuilder<Point3D<F>> {
    pub fn new_3d(nx: usize, ny: usize, nz: usize) -> Self {
        VertexCollectionBuilder {
            dimensions: Dimensions::Three { nx, ny, nz },
            write_order: None, 
            _phantom: PhantomData,
        }
    }

    pub fn set_f32(self) -> VertexCollectionBuilder<Point3D<f32>> {
        VertexCollectionBuilder {
            dimensions: self.dimensions,
            write_order: self.write_order,
            _phantom: PhantomData,
        }
    }

    pub fn set_f64(self) -> VertexCollectionBuilder<Point3D<f64>> {
        VertexCollectionBuilder {
            dimensions: self.dimensions,
            write_order: self.write_order,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_collection_builder() {
        let collection_2d = VertexCollectionBuilder::new_2d(9, 9)
            .set_write_order(WriteOrder::IJK)
            .set_f32()
            .build();
        assert!(collection_2d.is_ok());
    }
}