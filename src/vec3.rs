use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::prelude::Rng;
use std::cmp::PartialEq;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

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

    pub fn sqlen(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(self) -> f64 {
        self.sqlen().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self { x: self.y * other.z - self.z * other.y, 
               y: self.z * other.x - self.x * other.z, 
               z: self.x * other.y - self.y * other.x }
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self { x: rng.gen_range(min..max), y: rng.gen_range(min..max), z: rng.gen_range(min..max) }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p: Vec3 = Self { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: rng.gen_range(-1.0..1.0) };
            if p.sqlen() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p: Vec3 = Self { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: 0.0 };
            if p.sqlen() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(self) -> bool {
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

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[test]
fn test_gen() {
    let p = Vec3 {
        x: 0.1,
        y: 0.2,
        z: 0.3,
    };
    assert_eq!(p.x, 0.1);
    assert_eq!(p.y, 0.2);
    assert_eq!(p.z, 0.3);

    let q = Vec3::new(0.2, 0.3, 0.4);
    assert_eq!(q.x, 0.2);
    assert_eq!(q.y, 0.3);
    assert_eq!(q.z, 0.4);
}

#[test]
fn test_add() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    let q = Vec3::new(0.2, 0.3, 0.4);
    let r = p + q;
    assert_approx_eq!(r.x, 0.3);
    assert_approx_eq!(r.y, 0.5);
    assert_approx_eq!(r.z, 0.7);
}

#[test]
fn test_sub() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    let q = Vec3::new(0.2, 0.3, 0.4);
    let r = p - q;
    assert_approx_eq!(r.x, -0.1);
    assert_approx_eq!(r.y, -0.1);
    assert_approx_eq!(r.z, -0.1);
}

#[test]
fn test_neg() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    let q = -p;
    assert_approx_eq!(q.x, -0.1);
    assert_approx_eq!(q.y, -0.2);
    assert_approx_eq!(q.z, -0.3);
}

#[test]
fn test_mul() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    let q = Vec3::new(0.2, 0.3, 0.4);
    let r = p * q;
    assert_approx_eq!(r.x, 0.02);
    assert_approx_eq!(r.y, 0.06);
    assert_approx_eq!(r.z, 0.12);
}

#[test]
fn test_dot() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    let q = Vec3::new(0.2, 0.3, 0.4);
    assert_approx_eq!(p.dot(q), 0.2);
}

#[test]
fn test_sqlen() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    assert_approx_eq!(p.sqlen(), 0.14);
}

#[test]
fn test_random() {
    let p = Vec3::random(-1.0, 1.0);
    assert!(p.x >= -1.0 && p.x <= 1.0);
    assert!(p.y >= -1.0 && p.y <= 1.0);
    assert!(p.z >= -1.0 && p.z <= 1.0);
}

#[test]
fn test_near_zero() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    assert!(!p.near_zero());
    let p = Vec3::new(0.0, 0.0, 0.0);
    assert!(p.near_zero());
}