#![allow(dead_code)]

use crate::geometry::{points::Point, vertices::VertexCollection};

use super::nodes::Node;

pub struct Block<'a, P: Point> {
    pub id: usize, 
    pub vertex_collection: VertexCollection<P>,
    pub nodes: Vec<Node<'a, P>>,
}