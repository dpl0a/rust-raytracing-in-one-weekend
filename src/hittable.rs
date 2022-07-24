use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord<'mat> {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'mat Material,
}

impl<'mat> HitRecord<'mat> {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
	    self.front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
