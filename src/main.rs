use std::f64::INFINITY;
use rand::prelude::Rng;
use std::time::Instant;

use ray_tracing_weekend::vec3::{Vec3, Point3};
use ray_tracing_weekend::color::{Color, write_color};
use ray_tracing_weekend::ray::Ray;
use ray_tracing_weekend::hittable::Hittable;
use ray_tracing_weekend::hittable_list::HittableList;
use ray_tracing_weekend::sphere::Sphere;
use ray_tracing_weekend::camera::Camera;
use ray_tracing_weekend::material::Material;

fn ray_color(r: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    match world.hit(r, 0.001, INFINITY) {
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

fn main() {
    let start = Instant::now();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    // World
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();

    
    let material_ground: Material = Material::new_lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center: Material = Material::new_lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left: Material = Material::new_dielectric(1.5);
    let material_right: Material = Material::new_metal(Color::new(0.8, 0.6, 0.2), 0.0);

    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    object_list.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    object_list.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_left)));
    object_list.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    

    let world: Box<dyn Hittable> = Box::new(HittableList::new(object_list));

    // Camera
    let lookfrom: Point3 = Point3::new(3.0, 3.0, 2.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (lookfrom - lookat).len();
    let aperture: f64 = 2.0;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    // Initialize rng?
    let mut rng = rand::thread_rng();

    //Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} ", i);
        for j in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = ((j as f64) + rng.gen::<f64>()) / ((image_width - 1) as f64);
                let v: f64 = ((i as f64) + rng.gen::<f64>()) / ((image_height - 1) as f64);

                let r: Ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Fatto! Hai perso {} secondi della tua vita", start.elapsed().as_secs());
}