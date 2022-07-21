use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::prelude::Rng;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Self { e: [e0, e1, e2] }
    }

    pub fn sqlen(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn len(self) -> f64 {
        self.sqlen().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(self, other: Self) -> Self {
        Self { e: [self.e[1] * other.e[2] - self.e[2] * other.e[1], 
                   self.e[2] * other.e[0] - self.e[0] * other.e[2], 
                   self.e[0] * other.e[1] - self.e[1] * other.e[0]] }
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self { e: [rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max)] }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p: Vec3 = Self { e: [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)] };
            if p.sqlen() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(self) -> bool {
        let s: f64 = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
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

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, k: f64) -> Self::Output {
        Self { e: [self.e[0] - k, 
                   self.e[1] - k, 
                   self.e[2] - k] }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { e: [-self.e[0], 
                   -self.e[1], 
                   -self.e[2]] }
    }
}

impl Mul for Vec3 {
    type Output= Self;

    fn mul(self, other: Self) -> Self::Output {
        Self { e: [self.e[0] * other.e[0],
                   self.e[1] * other.e[1],
                   self.e[2] * other.e[2],] }
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
