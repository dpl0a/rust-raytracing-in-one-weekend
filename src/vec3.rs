use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn r(self) -> f64 {
        self.e[0]
    }

    pub fn g(self) -> f64 {
        self.e[1]
    }

    pub fn b(self) -> f64 {
        self.e[2]
    }

    pub fn sqlen(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(self) -> f64 {
        self.sqlen().sqrt()
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: Self, v: Self) -> Vec3 {
        Self { e: [u.e[1] * v.e[2] - u.e[2] * v.e[1], 
                   u.e[2] * v.e[0] - u.e[0] * v.e[2], 
                   u.e[0] * v.e[1] - u.e[1] * v.e[0]] }
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other : Self) -> Self::Output {
        Self { e: [self.e[0] + other.e[0], 
                   self.e[1] + other.e[1], 
                   self.e[2] + other.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self { e: [self.e[0] - other.e[0], 
                   self.e[1] - other.e[1], 
                   self.e[2] - other.e[2]] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, k: f64) -> Self::Output {
        Self { e: [k * self.e[0], 
                   k * self.e[1], 
                   k * self.e[2]] }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, k: f64) -> Self::Output {
        Self { e: [self.e[0] / k, 
                   self.e[1] / k, 
                   self.e[2] / k] }
    }
}