use crate::geometry::{line::Line, points::{Dimensioned, Point}};

pub struct Node<'a, P: Point> {
    id: usize, 
    north_face: Line<'a, P>,
    south_face: Line<'a, P>,
    east_face:  Line<'a, P>,
    west_face:  Line<'a, P>,
}

impl<'a, P: Point> Node<'a, P> {
    pub fn new(id: usize, nf: Line<'a, P>, sf: Line<'a, P>, ef: Line<'a, P>, wf: Line<'a, P>,) -> Self {
        Node { id, north_face: nf, south_face: sf, east_face: ef, west_face: wf }
    }
}

impl<'a, P: Point> Dimensioned for Node<'a, P> {
    fn is_2d(&self) -> bool {
        self.north_face.is_2d()
    }

    fn dimensions(&self) -> usize {
        self.north_face.dimensions()
    }
}

pub struct NodeCollection<'a, P: Point> {
    nodes: Vec<Node<'a, P>>,
}

impl<'a, P: Point> Dimensioned for NodeCollection<'a, P> {
    fn is_2d(&self) -> bool {
        if self.is_empty() {
            // return error
        }
        self.nodes.first().unwrap().is_2d()
    }

    fn dimensions(&self) -> usize {
        if self.is_empty() {
            // return error
        }
        self.nodes.first().unwrap().dimensions()
    }
}

impl<'a, P: Point> NodeCollection<'a, P> {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}