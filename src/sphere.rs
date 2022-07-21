use crate::vec3::{Vec3, Point3};
use crate::hittable::*;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Debug, Default, Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.sqlen();
        let b: f64 = oc.dot(r.direction);
        let c: f64 = oc.sqlen() - self.radius * self.radius;
        let discriminant: f64 = (b * b) - (a * c);

        if discriminant >= 0.0 {
            let sqrtd: f64 = discriminant.sqrt();
            let root_1: f64 = (-b - sqrtd) / a;
            let root_2: f64 = (-b + sqrtd) / a;

            for root in [root_1, root_2].iter() {
                if (*root < t_max) && (*root > t_min) {
                    let p = r.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = r.direction.dot(normal) < 0.0;

                    return Some(HitRecord {
                        t: *root,
                        p: p,
                        normal: if front_face { normal } else { -normal },
                        front_face: front_face,
                        material: &self.material,
                    });
                }
                
            }
        }
        None
    }
}
