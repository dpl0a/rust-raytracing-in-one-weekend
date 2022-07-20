mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;

use std::f64::INFINITY;
use rand::prelude::Rng;

use vec3::{Vec3, Point3};
use color::{Color, write_color};
use ray::Ray;
use hittable::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::Material;

fn ray_color(r: Ray, world: &Box<dyn Hittable>, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::default();

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Color = Color::default();
        if rec.material().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction: Vec3 = r.direction().normalize();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a: f64 = r.direction().sqlen();
    let b: f64 = Vec3::dot(oc, r.direction());
    let c: f64 = oc.sqlen() - radius * radius;
    let discriminant: f64 = b * b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / a;
    }
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    // World
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground: Material = Material::new_lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center: Material = Material::new_lambertian(Color::new(0.7, 0.3, 0.3));
    let material_left: Material = Material::new_metal(Color::new(0.8, 0.8, 0.8));
    let material_right: Material = Material::new_metal(Color::new(0.8, 0.6, 0.2));

    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    object_list.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    object_list.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let world: Box<dyn Hittable> = Box::new(HittableList::new(object_list));

    // Camera
    let cam = Camera::default();

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
            for s in 0..samples_per_pixel {
                let rand1: f64 = rng.gen();
                let rand2: f64 = rng.gen();
                let u: f64 = ((j as f64) + rand1) / ((image_width - 1) as f64);
                let v: f64 = ((i as f64) + rand2) / ((image_height - 1) as f64);

                let r: Ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done!");
}