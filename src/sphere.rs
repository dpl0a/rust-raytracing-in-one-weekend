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

pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
    let theta: f64 = (-p.y).acos();
    let phi: f64 = (-p.z / p.y).atan() - std::f64::consts::PI;
    (phi / (2.0 * std::f64::consts::PI), theta / std::f64::consts::PI)
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
                    let p: Point3 = r.at(*root);
                    let normal: Vec3 = (p - self.center) / self.radius;
                    let front_face: bool = r.direction.dot(normal) < 0.0;
                    let (u, v): (f64, f64) = get_sphere_uv(normal);

                    return Some(HitRecord {
                        t: *root,
                        u: u,
                        v: v,
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
