use std::time::Instant;
use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, Color};

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

fn ray_color(r: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some(rec) => {
            match rec.material.scatter(r, &rec) {
                Some((Some(scattered), attenuation)) => {
                    attenuation * ray_color(&scattered, world, depth - 1)
                }
                _ => {
                    Color::default()
                }
            }
        }
        None => {
            let unit_direction: Vec3 = r.direction.normalize();
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn render_line(pixels: &mut [(u8, u8, u8)], 
               camera: Camera, world: &Box<dyn Hittable>, 
               image_width: usize, image_height: usize, 
               samples_per_pixel: i32, 
               max_depth: i32, 
               y: usize) {
    let mut rng = rand::thread_rng();

    for x in 0..image_width {
        let mut pixel_color: Color = Color::default();
        for _ in 0..samples_per_pixel {
            let u: f64 = ((x as f64) + rng.gen::<f64>()) / ((image_width - 1) as f64);
            let v: f64 = ((y as f64) + rng.gen::<f64>()) / ((image_height - 1) as f64);
            let r: Ray = camera.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&r, &world, max_depth);
        }
        let scale: f64 = 1.0 / (samples_per_pixel as f64);
        let r: f64 = (scale * pixel_color.x).sqrt();
        let g: f64 = (scale * pixel_color.y).sqrt();
        let b: f64 = (scale * pixel_color.z).sqrt();

        pixels[x] = ((256.0 * clamp(r, 0.0, 0.999)) as u8, (256.0 * clamp(g, 0.0, 0.999)) as u8, (256.0 * clamp(b, 0.0, 0.999)) as u8);
    }
}

pub fn render(camera: Camera, world: &Box<dyn Hittable>, 
    image_width: usize, image_height: usize, 
    samples_per_pixel: i32, 
    max_depth: i32) {
        let mut pixels = vec![(0, 0, 0); image_width * image_height];
        let bands: Vec<(usize, &mut [(u8, u8, u8)])> = pixels.chunks_mut(image_width).enumerate().collect();

        bands.into_par_iter().for_each(|(i, band)| {
            render_line(band, camera, world, image_width, image_height, samples_per_pixel, max_depth, i);
            eprintln!("Line {} Rendered!", i);
        });

        for chunk in pixels.chunks(image_width).rev() {
            for p in chunk {
                println!("{} {} {}", p.0, p.1, p.2);
            }
        }
}  