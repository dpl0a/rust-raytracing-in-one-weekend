use crate::vec3::{Vec3, Point3};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3, t: f64) -> Self {
        Ray { origin: o, direction: d, time: t }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}