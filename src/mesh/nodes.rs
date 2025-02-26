#![allow(dead_code)]

use crate::geometry::prelude::*;

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

pub struct Nodes<'a, P: Point> {
    nodes: Vec<Node<'a, P>>,
}

impl<'a, P: Point> Dimensioned for Nodes<'a, P> {
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

impl<'a, P: Point> Nodes<'a, P> {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn add_node(&mut self, node: Node<'a, P>) {
        if node.dimensions() == self.dimensions() {
            self.nodes.push(node);
        }
    }
}

impl<'a> Nodes<'a, Point2D> {
    pub fn populate(&mut self, vertices: &'a Vertices<Point2D>) {
        let node_id: usize = 0; 
        for vertex in vertices.vertices() {
            let east_vertex = vertices.get_adjacent_vertex(vertex.get_id(), Direction::East);
            let north_vertex = vertices.get_adjacent_vertex(vertex.get_id(), Direction::North);
            if north_vertex.is_some() && east_vertex.is_some() {
                // get all adjacent vertices
                let east_vertex = east_vertex.unwrap();
                let north_vertex = north_vertex.unwrap();
                let northeast_vertex = vertices.get_adjacent_vertex(east_vertex.get_id(), Direction::North).unwrap();
                // construct the bounding lines
                let south = Line::new_2d(vertex, east_vertex);
                let west = Line::new_2d(vertex, north_vertex);
                let north = Line::new_2d(north_vertex, northeast_vertex);
                let east = Line::new_2d(east_vertex, northeast_vertex);
                // construct the node
                let node = Node::new(
                    node_id, 
                    north, 
                    south, 
                    east, 
                    west,
                );
                self.add_node(node);
            }
        }
    }
}