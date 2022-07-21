use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitRecord;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 }
}

pub fn reflect(u: Vec3, n: Vec3) -> Vec3 {
    u - n * Vec3::dot(u, n) * 2.0
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = Vec3::dot(-uv, n).min(1.0);
    let r_out_perp: Vec3 = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel: Vec3 = n * -(1.0 - r_out_perp.sqlen()).abs().sqrt();

    r_out_perp + r_out_parallel
}

impl Material {
    pub fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
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
            Self::Dielectric { refraction_index } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face() { *refraction_index } else { 1.0 / refraction_index };

                let unit_direction: Vec3 = r_in.direction().normalize();
                let cos_theta: f64 = Vec3::dot(-unit_direction, rec.normal()).min(1.0); 
                let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
                let mut direction: Vec3 = Vec3::default();

                if cannot_refract {
                    direction = reflect(unit_direction, rec.normal())
                } else {
                    direction = refract(unit_direction, rec.normal(), refraction_ratio)
                }

                *scattered = Ray::new(rec.p(), direction);
                return true;
            }
        }
    }

    pub fn new_lambertian(albedo: Color) -> Material {
        Self::Lambertian { albedo: albedo }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Material {
        Self::Metal { albedo: albedo, fuzz: fuzz }
    }

    pub fn new_dielectric(refraction_index: f64) -> Material {
        Self::Dielectric { refraction_index: refraction_index }
    }
}

impl Default for Material { // Stupid bodge
    fn default() -> Self {
        Self::Lambertian { albedo: Color::default() }
    }
}