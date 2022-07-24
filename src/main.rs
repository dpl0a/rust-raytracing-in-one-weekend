use std::time::Instant;
use std::env;

use ray_tracing_weekend::vec3::{Vec3, Point3};
use ray_tracing_weekend::hittable::Hittable;
use ray_tracing_weekend::camera::Camera;
use ray_tracing_weekend::raytracer::render;
use ray_tracing_weekend::scene::*;

fn main() {
    let start = Instant::now();

    // Read output filename
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <output_file.png>", args[0]);
        return;
    }
    let filename: &str = &args[1];

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    // World
    let world: Box<dyn Hittable> = Box::new(random_scene(true));

    // Camera
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

    render(filename, cam, &world, image_width, image_height, samples_per_pixel, max_depth);

    eprintln!("Fatto! Hai perso {} secondi della tua vita", start.elapsed().as_secs());
}
