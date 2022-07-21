use crate::vec3::{Vec3, Point3};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray {origin: o, direction: d}
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}