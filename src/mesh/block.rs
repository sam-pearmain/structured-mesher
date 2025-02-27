#![allow(dead_code)]

use crate::geometry::prelude::*;
use crate::mesh::nodes::Nodes;

type BoundaryFn2D = Box<dyn Fn(f64) -> f64>;        // y = f(x)
type BoundaryFn3D = Box<dyn Fn(f64) -> (f64, f64)>; // z = f(x, y)

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
    pub nodes: Nodes<'a, P>, 
}

pub struct BlockBuilder {
    id: usize,
    dimensions: Option<Dimensions>,
}

impl BlockBuilder {
    fn new(_id: usize) -> Self {
        todo!()
    }
}