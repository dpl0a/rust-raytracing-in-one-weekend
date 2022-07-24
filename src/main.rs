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
    //let world: Box<dyn Hittable> = Box::new(random_scene(true));
    let world: Box<dyn Hittable> = Box::new(two_spheres());

    // Camera
    //let cam = random_scene_cam(aspect_ratio);
    let cam = two_spheres_cam(aspect_ratio);

    render(filename, cam, &world, image_width, image_height, samples_per_pixel, max_depth);

    eprintln!("Fatto! Hai perso {} secondi della tua vita", start.elapsed().as_secs());
}
