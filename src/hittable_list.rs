use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::hittable::*;

pub struct Hittable_List {
   objects: Vec<Box<dyn Hittable>>
}

impl Hittable_List {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
	Self { objects }
    }
}

impl Hittable for Hittable_List {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut Hit_Record) -> bool {
	
	let mut temp_rec : Hit_Record = Hit_Record::default();
	let mut hit_anything : bool = false;
	let mut closest_so_far : f64 = t_max;
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
