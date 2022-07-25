use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

pub struct XYRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub material: Material,
}

pub struct XZRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Material,
}

pub struct YZRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Material,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: &Material) -> Self {
        Self { x0: x0, x1: x1, y0: y0, y1: y1, k: k, material: *material }
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: &Material) -> Self {
        Self { x0: x0, x1: x1, z0: z0, z1: z1, k: k, material: *material }
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: &Material) -> Self {
        Self { y0: y0, y1: y1, z0: z0, z1: z1, k: k, material: *material }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.origin.z) / r.direction.z;
        if (t < t_min) || (t > t_max) {
            return None;
        }
        let x: f64 = r.origin.x + t * r.direction.x;
        let y: f64 = r.origin.y + t * r.direction.y;
        if (x < self.x0) || (x > self.x1) || (y < self.y0) || (y > self.y1) {
            return None;
        }
        let u: f64 = (x - self.x0) / (self.x1 - self.x0);
        let v: f64 = (y - self.y0) / (self.y1 - self.y0);
        let normal: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        let p: Point3 = r.at(t);
        let front_face: bool = r.direction.dot(&normal) < 0.0;

        Some(HitRecord { 
            t: t,
            u: u,
            v: v,
            p: p,
            normal: if front_face { normal } else { -normal },
            front_face: front_face,
            material: &self.material,
         })
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.origin.y) / r.direction.y;
        if (t < t_min) || (t > t_max) {
            return None;
        }
        let x: f64 = r.origin.x + t * r.direction.x;
        let z: f64 = r.origin.z + t * r.direction.z;
        if (x < self.x0) || (x > self.x1) || (z < self.z0) || (z > self.z1) {
            return None;
        }
        let u: f64 = (x - self.x0) / (self.x1 - self.x0);
        let v: f64 = (z - self.z0) / (self.z1 - self.z0);
        let normal: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let p: Point3 = r.at(t);
        let front_face: bool = r.direction.dot(&normal) < 0.0;

        Some(HitRecord { 
            t: t,
            u: u,
            v: v,
            p: p,
            normal: if front_face { normal } else { -normal },
            front_face: front_face,
            material: &self.material,
         })
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.origin.x) / r.direction.x;
        if (t < t_min) || (t > t_max) {
            return None;
        }
        let y: f64 = r.origin.y + t * r.direction.y;
        let z: f64 = r.origin.z + t * r.direction.z;
        if (y < self.y0) || (y > self.y1) || (z < self.z0) || (z > self.z1) {
            return None;
        }
        let u: f64 = (y - self.y0) / (self.y1 - self.y0);
        let v: f64 = (z - self.z0) / (self.z1 - self.z0);
        let normal: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let p: Point3 = r.at(t);
        let front_face: bool = r.direction.dot(&normal) < 0.0;

        Some(HitRecord { 
            t: t,
            u: u,
            v: v,
            p: p,
            normal: if front_face { normal } else { -normal },
            front_face: front_face,
            material: &self.material,
         })
    }
}