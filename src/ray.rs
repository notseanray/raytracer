use crate::vec3::Vec3;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

pub struct Ray<F> {
    pub origin: Vec3<F>,
    pub direction: Vec3<F>,
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Display,
    > Ray<F>
{
    pub fn new(origin: Vec3<F>, direction: Vec3<F>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vec3<F> {
        self.origin
    }

    pub fn direction(&self) -> Vec3<F> {
        self.direction
    }

    pub fn at(&self, t: F) -> Vec3<F> {
        self.origin + self.direction * t
    }
}
