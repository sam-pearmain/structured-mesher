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

pub trait Point: Dimensioned {}

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

impl Point for Point2D {}

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

impl Point for Point3D {}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn at_origin() -> Self {
        Point3D { x: 0.0_f64, y: 0.0_f64, z: 0.0_f64 }
    }
}
