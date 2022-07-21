use crate::ray::Ray;
use crate::hittable::*;


pub struct HittableList {
   pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
	Self { objects: objects }
    }
}

impl Hittable for HittableList {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut closest_so_far: f64 = t_max;
		let mut hit_record = None;

		for object in self.objects.iter(){
			if let Some(rec) = object.hit(r, t_min, closest_so_far) {
				closest_so_far = rec.t;
				hit_record = Some(rec);
			}
		}
		hit_record
	}
}
