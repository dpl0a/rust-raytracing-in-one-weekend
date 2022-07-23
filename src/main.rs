use rand::prelude::Rng;
use std::time::Instant;
use std::env;

use ray_tracing_weekend::vec3::{Vec3, Point3, Color};
use ray_tracing_weekend::hittable::Hittable;
use ray_tracing_weekend::hittable_list::HittableList;
use ray_tracing_weekend::sphere::Sphere;
use ray_tracing_weekend::camera::Camera;
use ray_tracing_weekend::material::Material;
use ray_tracing_weekend::raytracer::*;

fn random_scene() -> HittableList {
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_material: Material = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center: Point3 = Point3::new((a as f64) + 0.9 * rng.gen::<f64>(), 0.2, (b as f64) + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Color = Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
                    let sphere_material: Material = Material::new_lambertian(albedo);
                    object_list.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo: Color = Color::random(0.5, 1.0);
                    let fuzz: f64 = rng.gen_range(0.0..0.5);
                    let sphere_material: Material = Material::new_metal(albedo, fuzz);
                    object_list.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material: Material = Material::new_dielectric(1.5);
                    object_list.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 : Material = Material::new_dielectric(1.5);
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), -0.95, material1)));

    let material2 : Material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    object_list.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 : Material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    object_list.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let world = HittableList::new(object_list);
    return world;
}

fn test_scene() -> HittableList {
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_material: Material = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let material1 : Material = Material::new_dielectric(1.5);
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), -0.95, material1)));

    let material2 : Material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    object_list.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 : Material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    object_list.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let world = HittableList::new(object_list);
    return world;
}

fn main() {
    let start = Instant::now();

    // read output filename
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <output_file.png>", args[0]);
        return;
    }
    let filename: &str = &args[1];

    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: usize = 400;
    let image_height: usize = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    // World
    // let world: Box<dyn Hittable> = Box::new(random_scene());
    let world: Box<dyn Hittable> = Box::new(test_scene());

    // Camera
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    render(filename, cam, &world, image_width, image_height, samples_per_pixel, max_depth);

    eprintln!("Fatto! Hai perso {} secondi della tua vita", start.elapsed().as_secs());
}
