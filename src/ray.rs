use crate::vec3::{Vec3, Point3};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray {origin: o, direction: d}
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[test]
fn test_ray() {
    let p = Vec3::new(0.1, 0.2, 0.3);
    let q = Vec3::new(0.2, 0.3, 0.4);

    let r = Ray::new(p, q);

    assert_approx_eq!(r.origin.x, 0.1);
    assert_approx_eq!(r.origin.y, 0.2);
    assert_approx_eq!(r.origin.z, 0.3);
    assert_approx_eq!(r.direction.x, 0.2);
    assert_approx_eq!(r.direction.y, 0.3);
    assert_approx_eq!(r.direction.z, 0.4);
}

#[test]
fn test_ray_at() {
    let p = Vec3::new(0.0, 0.0, 0.0);
    let q = Vec3::new(1.0, 2.0, 3.0);

    let r = Ray::new(p, q);
    let s = r.at(0.5);

    assert_approx_eq!(s.x, 0.5);
    assert_approx_eq!(s.y, 1.0);
    assert_approx_eq!(s.z, 1.5);
}