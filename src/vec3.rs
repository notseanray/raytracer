use std::{marker, ops::{Add, AddAssign, DivAssign, Index, MulAssign, Neg, Sub}};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3<F>([F; 3]);

pub type P3<F> = Vec3<F>;

impl <F: std::marker::Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F> + std::ops::Div<Output = F> + std::ops::Shr<i32, Output = F> + std::convert::From<f64>>Vec3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> F {
        self.0[0]
    }

    pub fn y(&self) -> F {
        self.0[1]
    }

    pub fn z(&self) -> F {
        self.0[2]
    }

    // https://suraj.sh/fast-square-root-approximation
    pub fn length(&self) -> F {
        let a = self.length_squared();
        let x = <f64 as Into<F>>::into(0x1fbd3f7d as f64)  + (a >> 1);
        (((x * x) + a) / x) * <f64 as Into<F>>::into(0.5)
    }

    pub fn length_squared(&self) -> F {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

impl <F: std::marker::Copy + std::ops::Add<Output = F> + std::ops::Mul<Output = F> + std::ops::Div<Output = F> + std::ops::Shr<i32, Output = F> + std::convert::From<f64>>Neg for Vec3<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x(), self.y(), self.z())
    }
}

impl <F>Index<usize> for Vec3<F> {
    type Output = F;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <F: std::marker::Copy + Add<Output = F> + std::ops::Mul<Output = F> + std::ops::Div<Output = F> + std::ops::Shr<i32, Output = F> + std::convert::From<f64>>AddAssign for Vec3<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()];
    }
}

impl <F: std::marker::Copy + Add<Output = F> + std::ops::Mul<Output = F> + std::ops::Div<Output = F> + std::ops::Shr<i32, Output = F> + std::convert::From<f64>>MulAssign for Vec3<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()];
    }
}

impl <F: std::marker::Copy + Add<Output = F> + std::ops::Mul<Output = F> + std::ops::Div<Output = F> + std::ops::Shr<i32, Output = F> + std::convert::From<f64>>DivAssign for Vec3<F> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = [self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z()];
    }
}
