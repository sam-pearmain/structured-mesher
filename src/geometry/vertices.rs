#![allow(dead_code)]

use std::fs::File;
use std::io::{Write, BufWriter};

use super::points::{Dimensioned, Point, Dimensions, Point2D, Point3D};
use super::vertex::Vertex;

pub enum Direction {
    North, // +j
    South, // -j
    East,  // +i
    West,  // -i
    Up,    // +k
    Down,  // -k
}

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

// methods for groups of 2D vertices
impl Vertices<Point2D> {
    pub fn new_2d(nx: usize, ny: usize) -> Vertices<Point2D> {
        Vertices { vertices: Vec::new(), dimensions: Dimensions::Two { nx, ny } }
    }

    pub fn get_adjacent_vertex(&self, vertex_id: usize, direction: Direction) -> Option<&Vertex<Point2D>> {
        let (i, j) = self.vertex_id_to_ij(vertex_id)?;
        let (nx, ny, _) = self.dimensions.as_tuple();

        let (new_i, new_j) = match direction {
            Direction::East => {
                if i + 1 >= nx { return None; } // eastern boundary
                (i + 1, j)
            }
            Direction::West => {
                if i == 0 { return None; } // western boundary
                (i - 1, j)
            }
            Direction::North => {
                if j + 1 >= ny { return None; } // northern boundary
                (i, j + 1)
            }
            Direction::South => {
                if j == 0 { return None; } // southern boundary
                (i, j - 1)
            }
            _ => return None,
        };

        let new_id = self.vertex_ij_to_id(new_i, new_j)?;
        self.get_vertex(new_id)
    }

    pub fn vertex_id_to_ij(&self, vertex_id: usize) -> Option<(usize, usize)> {
        if !self.vertex_exists(vertex_id) { return None; } // return none if the vertex doesn't exist
        let (nx, _, _) = self.dimensions.as_tuple();
        let i = vertex_id % nx;
        let j = vertex_id / nx;
        Some((i, j))
    }

    pub fn vertex_ij_to_id(&self, i: usize, j: usize) -> Option<usize> {
        let (nx, _, _) = self.dimensions.as_tuple();
        let vertex_id = (j * nx) + i;
        if self.vertex_exists(vertex_id) {
            Some(vertex_id)
        } else {
            None
        }
    }

    pub fn populate_uniform(&mut self) {
        let (nx, ny, _) = self.dimensions.as_tuple();
        let dx = 1.0 / (nx - 1) as f64;
        let dy = 1.0 / (ny - 1) as f64;
        
        for j in 0..ny {
            for i in 0..nx {
                let id = i + j * nx;
                let x = i as f64 * dx;
                let y = j as f64 * dy;
                self.add_vertex(Vertex::new_2d(id, x, y));
            }
        }
    }

    pub fn nx_ny(&self) -> (usize, usize) {
        let (nx, ny, _) = self.dimensions.as_tuple();
        (nx, ny)
    }
}

// methods for groups of 3D vertices
impl Vertices<Point3D> {
    pub fn new_3d(nx: usize, ny: usize, nz: usize) -> Vertices<Point3D> {
        Vertices { vertices: Vec::new(), dimensions: Dimensions::Three { nx, ny, nz } }
    } 

    pub fn get_adjacent_vertex(&self, vertex_id: usize, direction: Direction) -> Option<&Vertex<Point3D>> {
        let (i, j, k) = self.vertex_id_to_ijk(vertex_id)?;
        let (nx, ny, nz) = self.dimensions.as_tuple();

        let (new_i, new_j, new_k) = match direction {
            Direction::East => {
                if i + 1 >= nx { return None; } // eastern boundary
                (i + 1, j, k)
            }
            Direction::West => {
                if i == 0 { return None; } // western boundary
                (i - 1, j, k)
            }
            Direction::North => {
                if j + 1 >= ny { return None; } // northern boundary
                (i, j + 1, k)
            }
            Direction::South => {
                if j == 0 { return None; } // southern boundary
                (i, j - 1, k)
            }
            Direction::Up => {
                if k + 1 >= nz.unwrap() { return None; } // upper boundary
                (i, j, k + 1)
            }
            Direction::Down => {
                if k == 0 { return None; } // lower boundary
                (i, j, k - 1)
            }
        };

        let new_id = self.vertex_ijk_to_id(new_i, new_j, new_k)?;
        self.get_vertex(new_id)
    }

    pub fn vertex_id_to_ijk(&self, vertex_id: usize) -> Option<(usize, usize, usize)> {
        if !self.vertex_exists(vertex_id) { return None; } // return none if the vertex does not exist
        let (nx, ny, _) = self.dimensions.as_tuple();
        let i = vertex_id % nx;
        let j = (vertex_id / nx) % ny;
        let k = vertex_id / (nx * ny);
        Some((i, j, k))
    }

    pub fn vertex_ijk_to_id(&self, i: usize, j: usize, k: usize) -> Option<usize> {
        let (nx, ny, nz) = self.dimensions.as_tuple();
        if i >= nx || j >= ny || k >= nz.unwrap() { return None; } // vertex ijk is out of bounds
        let vertex_id = i + (j * nx) + (k * nx * ny);
        if self.vertex_exists(vertex_id) { Some(vertex_id) } else { None }
    }

    pub fn populate_uniform(&mut self) {
        let (nx, ny, nz) = self.dimensions.as_tuple();
        let nz = nz.unwrap();
        let dx = 1.0 / (nx - 1) as f64;
        let dy = 1.0 / (ny - 1) as f64;
        let dz = 1.0 / (nz - 1) as f64;
        
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let id = i + j * nx + k * nx * ny;
                    let x = i as f64 * dx;
                    let y = j as f64 * dy;
                    let z = k as f64 * dz;
                    self.add_vertex(Vertex::new_3d(id, x, y, z));
                }
            }
        }
    }
}

// shared methods that apply between both 2D and 3D lists of vertices
impl<P: Point> Vertices<P> {
    pub fn add_vertex(&mut self, vertex: Vertex<P>) {
        if vertex.dimensions() ==  self.dimensions() {
            self.vertices.push(vertex);
        }
    }

    pub fn get_vertex(&self, vertex_id: usize) -> Option<&Vertex<P>> {
        self.vertices.iter()
            .find(|&v| v.get_id() == vertex_id)
    }
    
    pub fn vertex_exists(&self, vertex_id: usize) -> bool {
        if self.get_vertex(vertex_id).is_some() {
            true
        } else {
            false
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex<P>> {
        &self.vertices
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

    #[test]
    fn test_populate_uniform() {
        let mut vertices_2d = Vertices::new_2d(3, 2);
        vertices_2d.populate_uniform();
        vertices_2d.export_csv("2d.csv").expect("erm");
        assert_eq!(vertices_2d.vertices.len(), 6);

        let mut vertices_3d = Vertices::new_3d(2, 2, 2);
        vertices_3d.populate_uniform();
        vertices_3d.export_csv("3d.csv").expect("erm");
        assert_eq!(vertices_3d.vertices.len(), 8);
    }
}