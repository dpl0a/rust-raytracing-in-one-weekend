use crate::ray::Ray;
use crate::hittable::*;

pub struct HittableList {
   objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
	Self { objects: objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
	let mut temp_rec: HitRecord = HitRecord::default();
	let mut hit_anything: bool = false;
	let mut closest_so_far: f64 = t_max;

	for object in &self.objects {
	    if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
		hit_anything = true;
		closest_so_far = temp_rec.t();
		*rec = temp_rec;
	    }
	}

	hit_anything
    } 
	
}
