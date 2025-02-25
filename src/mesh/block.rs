#![allow(dead_code)]

use std::marker::PhantomData;
use crate::geometry::prelude::*;
use crate::mesh::nodes::{Node, NodeCollection};

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
    pub vertices: VertexCollection<P>,
    pub nodes: NodeCollection<'a, P>, 
}

pub struct BlockBuilder<'a, P: Point> {
    id: usize, 
    dimensions: Option<Dimensions>,
    vertex_collection_builder: Option<VertexCollectionBuilder<P>>,
    node_collection_builder: Option<NodeCollection<'a, P>>,
    _phantom: PhantomData<P>,
}

impl<'a, P: Point> BlockBuilder<'a, P> {
    fn new(id: usize) -> Self {
        BlockBuilder {
            id, 
            dimensions: None, 
            vertex_collection_builder: None,
            node_collection_builder: None, 
            _phantom: PhantomData,
        }
    }
}