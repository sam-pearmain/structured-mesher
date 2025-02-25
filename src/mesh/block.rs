#![allow(dead_code)]

use std::marker::PhantomData;
use crate::geometry::prelude::*;
use crate::mesh::nodes::Node;

pub enum BoundaryType {
    Vertical, 
    Horizontal, 

}

pub struct Block<'a, P: Point> {
    pub id: usize, 
    pub vertex_collection: VertexCollection<P>,
    pub nodes: Vec<Node<'a, P>>, // should probably be a node collection
}

pub struct BlockBuilder<P: Point> {
    _phantom: PhantomData<P>,
}