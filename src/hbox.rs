use crate::vec3::{Point3};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::rectangle::*;
use crate::material::Material;

pub struct HBox {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl HBox {
    pub fn new(min: Point3, max: Point3, material: Material) -> Self {
        let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();

        object_list.push(Box::new(XYRect::new(min.x, max.x, min.y, max.y, max.z, &material)));
        object_list.push(Box::new(XYRect::new(min.x, max.x, min.y, max.y, min.z, &material)));
        object_list.push(Box::new(XZRect::new(min.x, max.x, min.z, max.z, max.y, &material)));
        object_list.push(Box::new(XZRect::new(min.x, max.x, min.z, max.z, min.y, &material)));
        object_list.push(Box::new(YZRect::new(min.y, max.y, min.z, max.z, max.x, &material)));
        object_list.push(Box::new(YZRect::new(min.y, max.y, min.z, max.z, min.x, &material)));

        let sides = HittableList::new(object_list);

        Self { min: min, max: max, sides: sides }
    }
}

impl Hittable for HBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}