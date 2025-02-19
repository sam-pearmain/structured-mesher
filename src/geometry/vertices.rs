use super::points::Dimensioned;
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

pub struct VertexCollectionBuilder {
    dimensions: Dimensions,
    write_order: WriteOrder,
}

impl VertexCollectionBuilder {
    fn build<P: Point>(self) -> VertexCollection<P> {
        if self.dimensions.is_2d() {
            VertexCollection {
                vertices: Vec::new(),
                dimensions: self.dimensions,
                write_order: self.write_order,
            }
        } else {
            VertexCollection { 
                vertices: Vec::new(),
                dimensions: self.dimensions,
                write_order: self.write_order,
            }
        }
    }

    fn set_2d(mut self, nx: usize, ny: usize) -> Self {
        self.dimensions = Dimensions::Two { nx: nx, ny: ny };
        self
    }

    fn set_3d(mut self, nx: usize, ny: usize, nz: usize) -> Self {
        self.dimensions = Dimensions::Three { nx: nx, ny: ny, nz: nz };
        self
    }

    fn set_write_order(mut self, write_order: WriteOrder) -> Self {
        self.write_order = write_order;
        self
    }
}