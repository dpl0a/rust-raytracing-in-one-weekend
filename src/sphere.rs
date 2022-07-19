use crate::vec3::{Vec3, Point3};
use crate::hittable::*;
use crate::ray::Ray;

#[derive(Debug, Default, Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn sphere(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut Hit_Record) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().sqlen();
        let b: f64 = Vec3::dot(oc, r.direction());
        let c: f64 = oc.sqlen() - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
    
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (-b -sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return false;
            }
        }
        rec.set_t(root);
        rec.set_p(r.at(rec.t()));
        rec.set_normal((rec.p() - self.center) / self.radius);

        return true;
    }
}