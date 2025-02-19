use super::prelude::*;

pub trait Dimensioned {
    fn is_2d(&self) -> bool;
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point2D<F: Float> {
    x: F,
    y: F,
}

impl<F: Float> Dimensioned for Point2D<F> {
    fn is_2d(&self) -> bool {
        true
    }
}

impl<F: Float> Point2D<F> {
    fn new(x: F, y: F) -> Self {
        Point2D { x: x, y: x }
    }

    fn at_origin() -> Self {
        Point2D { x: F::zero(), y: F::zero() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3d<F: Float> {
    x: F, 
    y: F, 
    z: F,
}

impl<F: Float> Dimensioned for Point3d<F> {
    fn is_2d(&self) -> bool {
        false
    }
}

impl<F: Float> Point3d<F> {
    fn new(x: F, y: F, z: F) -> Self {
        Point3d { x: x, y: y, z: z }
    }

    fn at_origin() -> Self {
        Point3d { x: F::zero(), y: F::zero(), z: F::zero() }
    }
}