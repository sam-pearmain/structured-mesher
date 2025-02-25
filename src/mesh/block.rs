#![allow(dead_code)]

use std::marker::PhantomData;
use crate::geometry::prelude::*;
use crate::mesh::nodes::NodeCollection;

pub enum BoundaryType {
    North,  // 2D
    South,  // 2D
    East,   // 2D
    West,   // 2D
    Top,    // 3D
    Bottom, // 3D
}

pub struct Block<'a, P: Point> {
    pub id: usize, 
    pub vertices: Vertices<P>,
    pub nodes: NodeCollection<'a, P>, 
}

pub struct BlockBuilder<P: Point> {
    id: usize, 
    dimensions: Option<Dimensions>,
    _phantom: PhantomData<P>,
}

impl<'a, P: Point> BlockBuilder<P> {
    fn new(id: usize) -> Self {
        BlockBuilder {
            id, 
            dimensions: None,
            _phantom: PhantomData,
        }
    }
}