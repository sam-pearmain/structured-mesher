use super::prelude::*;

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
pub struct Point2D<F: Float> {
    x: F,
    y: F,
}

impl<F: Float> Dimensioned for Point2D<F> {
    fn is_2d(&self) -> bool {
        true
    }

    fn dimensions(&self) -> usize {
        2
    }
}

impl<F: Float> Point2D<F> {
    pub fn new(x: F, y: F) -> Self {
        Point2D { x: x, y: y }
    }

    pub fn at_origin() -> Self {
        Point2D { x: F::zero(), y: F::zero() }
    }
}

impl<F: Float> Point for Point2D<F> {}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3D<F: Float> {
    x: F, 
    y: F, 
    z: F,
}

impl<F: Float> Dimensioned for Point3D<F> {
    fn is_2d(&self) -> bool {
        false
    }

    fn dimensions(&self) -> usize {
        3
    }
}

impl<F: Float> Point3D<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Point3D { x: x, y: y, z: z }
    }

    pub fn at_origin() -> Self {
        Point3D { x: F::zero(), y: F::zero(), z: F::zero() }
    }
}

impl<F: Float> Point for Point3D<F> {}