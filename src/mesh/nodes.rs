#![allow(dead_code)]

use crate::geometry::prelude::*;

pub struct Node<'a, P: Point> {
    pub id: usize, 
    pub north_face: Line<'a, P>,
    pub south_face: Line<'a, P>,
    pub east_face:  Line<'a, P>,
    pub west_face:  Line<'a, P>,
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
    pub nodes: Vec<Node<'a, P>>,
}

impl<'a, P: Point> Dimensioned for Nodes<'a, P> {
    fn is_2d(&self) -> bool {
        if self.is_empty() {
            true  // Default to true for empty collections
        } else {
            self.nodes.first().unwrap().is_2d()
        }
    }

    fn dimensions(&self) -> usize {
        if self.is_empty() {
            2  // Default to 2D for empty collections
        } else {
            self.nodes.first().unwrap().dimensions()
        }
    }
}

impl<'a, P: Point> Nodes<'a, P> {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn add_node(&mut self, node: Node<'a, P>) {
        if self.is_empty() || node.dimensions() == self.dimensions() {
            self.nodes.push(node);
        }
    }
}

impl<'a> Nodes<'a, Point2D> {
    pub fn new_2d() -> Self {
        Nodes { nodes: Vec::new() }
    }

    pub fn populate(&mut self, vertices: &'a Vertices<Point2D>) -> Result<(), &'static str> {
        if !vertices.is_2d() {
            return Err("Cannot populate 2D nodes from non-2D vertices");
        }

        let mut node_id: usize = 0;
        for vertex in vertices.vertices() {
            // Get adjacent vertices
            let east_vertex = vertices.get_adjacent_vertex(vertex.get_id(), Direction::East);
            let north_vertex = vertices.get_adjacent_vertex(vertex.get_id(), Direction::North);
            
            // Only create node if we have both east and north vertices
            if let (Some(east), Some(north)) = (east_vertex, north_vertex) {
                // Get northeast vertex, required to complete the node
                let northeast_vertex = vertices
                    .get_adjacent_vertex(east.get_id(), Direction::North)
                    .ok_or("Failed to get northeast vertex")?;

                // Construct the bounding lines
                let south = Line::new_2d(vertex, east);
                let west = Line::new_2d(vertex, north);
                let north = Line::new_2d(north, northeast_vertex);
                let east = Line::new_2d(east, northeast_vertex);

                // Construct and add the node
                let node = Node::new(node_id, north, south, east, west);
                self.add_node(node);
                node_id += 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate_2d_nodes() {
        let mut vertices = Vertices::new_2d(3, 3);
        vertices.populate_uniform();

        let mut nodes = Nodes::new_2d();
        assert!(nodes.populate(&vertices).is_ok());

        assert_eq!(nodes.nodes.len(), 4);
        
        for (i, node) in nodes.nodes.iter().enumerate() {
            assert_eq!(node.id, i);
        }
    }
}