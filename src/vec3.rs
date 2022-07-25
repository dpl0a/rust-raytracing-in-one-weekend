use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::prelude::Rng;
use std::cmp::PartialEq;
use crate::PRNG;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Self { x: x, y: y, z: z }
    }

    pub fn sqlen(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.sqlen().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self { x: self.y * other.z - self.z * other.y, 
               y: self.z * other.x - self.x * other.z, 
               z: self.x * other.y - self.y * other.x }
    }

    pub fn normalize(&self) -> Self {
        *self / self.len()
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self { x: rng.gen_range(min..max), y: rng.gen_range(min..max), z: rng.gen_range(min..max) }
    }

    pub fn random_in_unit_sphere(rng: &mut PRNG) -> Self {
        //let mut rng = rand::thread_rng();
        loop {
            let p: Vec3 = Self { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: rng.gen_range(-1.0..1.0) };
            if p.sqlen() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk(rng: &mut PRNG) -> Self {
        //let mut rng = rand::thread_rng();
        loop {
            let p: Vec3 = Self { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: 0.0 };
            if p.sqlen() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self { x: self.x + other.x, 
               y: self.y + other.y, 
               z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self { x: self.x - other.x, 
               y: self.y - other.y, 
               z: self.z - other.z }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, k: f64) -> Self::Output {
        Self { x: self.x - k, 
               y: self.y - k, 
               z: self.z - k }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, 
               y: -self.y, 
               z: -self.z }
    }
}

impl Mul for Vec3 {
    type Output= Self;

    fn mul(self, other: Self) -> Self::Output {
        Self { x: self.x * other.x,
               y: self.y * other.y,
               z: self.z * other.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, k: f64) -> Self::Output {
        Self { x: k * self.x, 
               y: k * self.y, 
               z: k * self.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, k: f64) -> Self::Output {
        Self { x: self.x / k, 
               y: self.y / k, 
               z: self.z / k }
    }
}