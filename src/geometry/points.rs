#![allow(dead_code)]

pub trait Dimensioned {
    fn is_2d(&self) -> bool;
    fn dimensions(&self) -> usize;
}

#[derive(Debug)]
pub enum Dimensions {
    Two { nx: usize, ny: usize },
    Three {nx: usize, ny: usize, nz: usize }
}

impl Dimensioned for Dimensions {
    fn is_2d(&self) -> bool {
        matches!(self, Dimensions::Two { .. })
    }

    fn dimensions(&self) -> usize {
        match self {
            Dimensions::Two { .. } => 2,
            Dimensions::Three { .. } => 3,
        }
    }
}

impl Dimensions {
    pub fn total_points(&self) -> usize {
        match self {
            Dimensions::Two { nx, ny } => nx * ny,
            Dimensions::Three { nx, ny, nz } => nx * ny * nz,
        }
    }
}

pub trait Point: Dimensioned {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Dimensioned for Point2D {
    fn is_2d(&self) -> bool {
        true
    }

    fn dimensions(&self) -> usize {
        2
    }
}

impl Point for Point2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        panic!("z coordinate accessed for 2D point")
    }
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    pub fn at_origin() -> Self {
        Point2D { x: 0.0_f64, y: 0.0_f64 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3D {
    x: f64, 
    y: f64, 
    z: f64,
}

impl Dimensioned for Point3D {
    fn is_2d(&self) -> bool {
        false
    }

    fn dimensions(&self) -> usize {
        3
    }
}

impl Point for Point3D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn at_origin() -> Self {
        Point3D { x: 0.0_f64, y: 0.0_f64, z: 0.0_f64 }
    }
}
