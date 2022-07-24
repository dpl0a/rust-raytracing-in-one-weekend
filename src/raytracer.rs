use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;

use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128Plus;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Vec3, Color};
use crate::PRNG;

fn write_image(filename: &str, 
               pixels: &[u8],
	           image_width: usize, 
               image_height: usize) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(pixels, image_width as u32, image_height as u32, ColorType::Rgb8).expect("error writing image: PngEncoder::write_image error");
    Ok(())
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

fn ray_color(r: &Ray, background: Color, world: &Box<dyn Hittable>, depth: i32, rng: &mut PRNG) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some(rec) => {
            match rec.material.scatter(r, &rec, rng) {
                Some((scattered, attenuation)) => {
                    attenuation * ray_color(&scattered, background, world, depth - 1, rng)
                }
                _ => {
                    rec.material.emitted(rec.u, rec.v, rec.p)
                }
            }
        }
        None => {
            background
        }
    }
}

fn render_line(pixels: &mut [u8], camera: Camera, 
               world: &Box<dyn Hittable>, 
               background: Color,
               image_width: usize, image_height: usize, 
               samples_per_pixel: i32, 
               max_depth: i32, 
               y: usize) {

    let mut rng = Xoroshiro128Plus::from_entropy();

    for x in 0..image_width {
        let mut pixel_color: Color = Color::default();
        for _ in 0..samples_per_pixel {
            let u: f64 = ((x as f64) + rng.gen::<f64>()) / ((image_width - 1) as f64);
            let v: f64 = (image_height as f64 - ((y as f64) + rng.gen::<f64>())) / ((image_height - 1) as f64);
            let r: Ray = camera.get_ray(u, v, &mut rng);
            pixel_color = pixel_color + ray_color(&r, background, &world, max_depth, &mut rng);
        }
        let scale: f64 = 1.0 / (samples_per_pixel as f64);
        let r: f64 = (scale * pixel_color.x).sqrt();
        let g: f64 = (scale * pixel_color.y).sqrt();
        let b: f64 = (scale * pixel_color.z).sqrt();

        pixels[x * 3]     = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        pixels[x * 3 + 1] = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        pixels[x * 3 + 2] = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    }
}

pub fn render(filename: &str, 
              camera: Camera, 
              world: &Box<dyn Hittable>, 
              background: Color,
              image_width: usize, 
              image_height: usize, 
              samples_per_pixel: i32, 
              max_depth: i32) {
    let mut pixels = vec![0; image_width * image_height * 3];
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(image_width * 3).enumerate().collect();

    bands.into_par_iter().for_each(|(i, band)| {
        render_line(band, camera, world, background, image_width, image_height, samples_per_pixel, max_depth, i);
        eprintln!("Line {} Rendered!", i);
    });
    write_image(filename, &pixels, image_width, image_height).expect("error writing image: std::io::Error");

}  
