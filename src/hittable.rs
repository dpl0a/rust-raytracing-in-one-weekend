use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

#[derive(Debug, Copy, Clone, Default)]
pub struct HitRecord {
    t: f64,
    p: Point3,
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    pub fn p(self) -> Point3 {
        self.p
    }

    pub fn t(self) -> f64 {
        self.t
    }

    pub fn normal(self) -> Vec3 {
        self.normal
    }

    pub fn set_p(&mut self, val: Point3) {
        self.p = val
    }

    pub fn set_t(&mut self, val: f64) {
        self.t = val
    }

    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
	self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;

	if self.front_face {
	    self.normal = outward_normal
	} else {
	    self.normal = -outward_normal
	}
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        return false;
    }
}
