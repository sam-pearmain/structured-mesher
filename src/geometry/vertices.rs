#![allow(dead_code)]

use std::fs::File;
use std::io::{Write, BufWriter};

use super::points::{Dimensioned, Point, Dimensions, Point2D, Point3D};
use super::vertex::Vertex;

pub struct Vertices<P: Point> {
    vertices: Vec<Vertex<P>>,
    dimensions: Dimensions,
}

impl<P: Point> Dimensioned for Vertices<P> {
    fn is_2d(&self) -> bool {
        self.dimensions.is_2d()
    }

    fn dimensions(&self) -> usize {
        self.dimensions.dimensions()
    }
}

impl Vertices<Point2D> {
    pub fn new_2d(nx: usize, ny: usize) -> Vertices<Point2D> {
        Vertices { vertices: Vec::new(), dimensions: Dimensions::Two { nx, ny } }
    }
}

impl Vertices<Point3D> {
    pub fn new_3d(nx: usize, ny: usize, nz: usize) -> Vertices<Point3D> {
        Vertices { vertices: Vec::new(), dimensions: Dimensions::Three { nx, ny, nz } }
    } 
}

impl<P: Point> Vertices<P> {
    pub fn add_vertex(&mut self, vertex: Vertex<P>) {
        if vertex.dimensions() ==  self.dimensions() {
            self.vertices.push(vertex);
        }
    }

    pub fn get_vertex(&self, vertex_id: usize) -> Option<&Vertex<P>> {
        self.vertices.iter()
            .find(|&v| v.id == vertex_id)
    }

    pub fn export_csv(&self, filename: &str) -> Result<(), &'static str> {
        let file = File::create(filename).map_err(|_| "failed to create file")?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "id,x,y{}", if self.is_2d() { "" } else { ",z" })
            .map_err(|_| "failed to write header")?;

        for vertex in &self.vertices {
            if self.is_2d() {
                writeln!(
                    writer,
                    "{},{},{}",
                    vertex.get_id(),
                    vertex.get_x(),
                    vertex.get_y(),
                )
            } else {
                writeln!(
                    writer,
                    "{},{},{},{}",
                    vertex.get_id(),
                    vertex.get_x(),
                    vertex.get_y(),
                    vertex.get_z(),
                )
            }.map_err(|_| "failed to write vertex data")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_creation() {
        let vertices_2d = Vertices::new_2d(10, 10);
        let vertices_3d = Vertices::new_3d(10, 10, 10);
        assert!(vertices_2d.is_2d());
        assert!(!vertices_3d.is_2d());
    }

    #[test]
    fn test_adding_vertices() {
        let mut vertices_2d = Vertices::new_2d(10, 10);
        let mut vertices_3d = Vertices::new_3d(10, 10, 10);
        vertices_2d.add_vertex(Vertex::new_2d(0, 1.0, 2.0));
        vertices_3d.add_vertex(Vertex::new_3d(0, 1.0, 2.0, 3.0));

        assert_eq!(vertices_2d.vertices.len(), 1);
        assert_eq!(vertices_3d.vertices.len(), 1);

        let v2d = &vertices_2d.get_vertex(0).unwrap();
        assert_eq!(v2d.get_id(), 0);
        assert_eq!(v2d.get_x(), 1.0);
        assert_eq!(v2d.get_y(), 2.0);

        let v3d = &vertices_3d.get_vertex(0).unwrap();
        assert_eq!(v3d.get_id(), 0);
        assert_eq!(v3d.get_x(), 1.0);
        assert_eq!(v3d.get_y(), 2.0);
        assert_eq!(v3d.get_z(), 3.0);
    }

    #[test]
    fn test_export_csv() {
        let mut vertices_2d = Vertices::new_2d(2, 2);
        vertices_2d.add_vertex(Vertex::new_2d(0, 1.0, 2.0));
        vertices_2d.add_vertex(Vertex::new_2d(1, 3.0, 4.0));
        
        let filename = "test_vertices_2d.csv";
        assert!(vertices_2d.export_csv(filename).is_ok());
        
        let contents = fs::read_to_string(filename)
            .expect("should be able to read the file");
        
        assert!(contents.contains("id,x,y"));
        assert!(contents.contains("0,1,2"));
        assert!(contents.contains("1,3,4"));
        
        fs::remove_file(filename).expect("failed to clean up test file");
    }
}