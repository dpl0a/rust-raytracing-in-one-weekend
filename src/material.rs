use rand::prelude::Rng;

use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::PRNG;
use crate::texture::Texture;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Textured { texture: Texture },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
    Light { albedo: Color },
}


fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = ((-uv).dot(n)).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * (-1.0 * (1.0 - r_out_perp.sqlen()).abs().sqrt());
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material {
    pub fn scatter(&self, r: &Ray, rec: &HitRecord, rng: &mut PRNG) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction: Vec3 = rec.normal + Vec3::random_in_unit_sphere(rng);
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered: Ray = Ray::new(rec.p, scatter_direction, r.time);
                let attenuation: Color = *albedo;
                Some((scattered, attenuation))
            }
            Self::Textured { texture } => {
                let mut scatter_direction: Vec3 = rec.normal + Vec3::random_in_unit_sphere(rng);
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered: Ray = Ray::new(rec.p, scatter_direction, r.time);
                let attenuation: Color = texture.value(rec.u, rec.v, rec.p);
                Some((scattered, attenuation))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected: Vec3 = reflect(r.direction, rec.normal);
                let scattered: Ray = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere(rng) * *fuzz, r.time);
                let attenuation: Color = *albedo;
                if scattered.direction.dot(rec.normal) > 0.0 {
                    return Some((scattered, attenuation));
                }
                None
            }
            Self::Dielectric { refraction_index } => {
                let attenuation: Color = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio: f64 = if rec.front_face { 1.0 / refraction_index } else { *refraction_index };
                let unit_direction = r.direction.normalize();
                let cos_theta: f64 = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta: f64 = (1.0 - (cos_theta * cos_theta)).sqrt();
                let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

                if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
                    let reflected: Vec3 = reflect(unit_direction, rec.normal);
                    let scattered: Ray = Ray::new(rec.p, reflected, r.time);
                    Some((scattered, attenuation))
                } else {
                    let direction: Vec3 = refract(unit_direction, rec.normal, refraction_ratio);
                    let scattered: Ray = Ray::new(rec.p, direction, r.time);
                    Some((scattered, attenuation))
                }
            }
            Self::Light { albedo: _albedo } => {
                None
            }
        }
    }

    pub fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        match self {
            Self::Light { albedo } => {
                *albedo
            }
            _ => {
                Color::default()
            }
        }
    }

    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian { albedo: albedo }
    }

    pub fn new_textured(texture: Texture) -> Self {
        Self::Textured { texture: texture }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal { albedo: albedo, fuzz: fuzz }
    }

    pub fn new_dielectric(refraction_index: f64) -> Self {
        Self::Dielectric { refraction_index: refraction_index }
    }

    pub fn new_light(albedo: Color) -> Self {
        Self::Light { albedo: albedo }
    }
}

impl Default for Material { // Stupid bodge
    fn default() -> Self {
        Self::Lambertian { albedo: Color::default() }
    }
}