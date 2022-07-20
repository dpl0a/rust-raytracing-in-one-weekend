use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitRecord;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

pub fn reflect(u: Vec3, n: Vec3) -> Vec3 {
    u - n * Vec3::dot(u, n) * 2.0
}

impl Material {
    pub fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match &self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction: Vec3 = rec.normal() + Vec3::random_in_unit_sphere();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal()
                }

                *scattered = Ray::new(rec.p(), scatter_direction);
                *attenuation = *albedo;
                return true;
            }
            Self::Metal { albedo, fuzz } => {
                let reflected: Vec3 = reflect(r_in.direction(), rec.normal());
                *scattered = Ray::new(rec.p(), reflected + Vec3::random_in_unit_sphere() * *fuzz );
                *attenuation = *albedo;
                Vec3::dot(scattered.direction(), rec.normal()) > 0.0
            }
        }
    }

    pub fn new_lambertian(albedo: Color) -> Material {
        Self::Lambertian { albedo: albedo }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Material {
        Self::Metal { albedo: albedo, fuzz: fuzz }
    }
}

impl Default for Material { // Stupid bodge
    fn default() -> Self {
        Self::Lambertian { albedo: Color::default() }
    }
}