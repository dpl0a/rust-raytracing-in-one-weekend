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
	    self.front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Self {
        Self { object: object, offset: offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r: Ray = Ray::new(&(r.origin - self.offset), &r.direction, r.time);
        match self.object.hit(&moved_r, t_min, t_max) {
            Some(rec) => {
                let front_face: bool = moved_r.direction.dot(&rec.normal) < 0.0;
                let p = rec.p + self.offset;

                Some(HitRecord {
                    t: rec.t,
                    u: rec.u,
                    v: rec.v,
                    p: p,
                    normal: if front_face { rec.normal } else { -rec.normal },
                    front_face: front_face,
                    material: rec.material,
                })
            }
            None => None
        }
    }
}

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, angle: f64) -> Self {
        let radians: f64 = angle.to_radians();

        Self { object: object, sin_theta: radians.sin(), cos_theta: radians.cos() }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin: Point3 = Point3::new(self.cos_theta * r.origin.x - self.sin_theta * r.origin.z, 
                                             r.origin.y, 
                                             self.sin_theta * r.origin.x + self.cos_theta * r.origin.z);
        let direction: Vec3 = Vec3::new(self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
                                            r.direction.y,
                                            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z);

        let rotated_r: Ray = Ray::new(&origin, &direction, r.time);
        match self.object.hit(&rotated_r, t_min, t_max) {
            Some (rec) => {
                let p: Point3 = Point3::new(self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                                            rec.p.y,
                                            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z);
                let normal: Point3 = Point3::new(self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                                                 rec.normal.y,
                                                 -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z);
                let front_face: bool = rotated_r.direction.dot(&normal) < 0.0;

                Some(HitRecord {
                    t: rec.t,
                    u: rec.u,
                    v: rec.v,
                    p: p,
                    normal: if front_face { normal } else { -normal },
                    front_face: front_face,
                    material: rec.material,
                })
            }
            None => None
        }
    }
}