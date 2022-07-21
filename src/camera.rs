use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self { origin: origin, horizontal: horizontal, vertical: vertical, lower_left_corner: lower_left_corner }
    }
}