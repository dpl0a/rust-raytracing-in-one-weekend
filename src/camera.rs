use rand::prelude::Rng;

use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::PRNG;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,

}

impl Camera {
    pub fn new(lookfrom: &Point3, 
               lookat: &Point3, vup: &Vec3, 
               vfov: f64, 
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64,
               t0: f64,
               t1: f64) -> Self {
        let theta: f64 = vfov.to_radians();
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = (*lookfrom - *lookat).normalize();
        let u: Vec3 = Vec3::cross(&vup, &w).normalize();
        let v: Vec3 = Vec3::cross(&w, &u);

        let origin: Point3 = *lookfrom;
        let horizontal: Vec3 = u * viewport_width * focus_dist;
        let vertical: Vec3 = v * viewport_height * focus_dist;

        Self { origin: origin,
               horizontal: horizontal,
               vertical: vertical,
               lower_left_corner: origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist,
               u: u,
               v: v,
               w: w,
               lens_radius: aperture / 2.0,
               time0: t0,
               time1: t1,
             }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut PRNG) -> Ray {
        let rd: Vec3 = Vec3::random_in_unit_disk(rng) * self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        Ray::new(&(self.origin + offset), 
                 &(self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset),
                 rng.gen_range(self.time0..self.time1))
    }
}