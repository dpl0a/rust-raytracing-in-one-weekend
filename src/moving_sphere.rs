use crate::vec3::{Vec3, Point3};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,
}

impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, time0: f64, time1: f64, radius: f64, material: Material) -> Self {
        Self { center0: center0,
               center1: center1,
               time0: time0,
               time1: time1,
               radius: radius,
               material: material }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + ((self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center(r.time);
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
                    let normal = (p - self.center(r.time)) / self.radius;
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