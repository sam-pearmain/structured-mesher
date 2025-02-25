#![allow(dead_code)]

use num_traits::Float;
use std::marker::PhantomData;

use crate::geometry::{points::{Point, Point2D}, vertices::VertexCollection};

use super::nodes::Node;

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