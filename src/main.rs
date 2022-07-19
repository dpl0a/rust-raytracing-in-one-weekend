mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

use std::f64::INFINITY;

use vec3::{Vec3, Point3};
use color::{Color, write_color};
use ray::Ray;
use hittable::Hit_Record;
use hittable::Hittable;
use hittable_list::Hittable_List;
use sphere::Sphere;

fn ray_color(r: Ray, world: &Box<dyn Hittable>) -> Color {
    let mut rec : Hit_Record = Hit_Record::default();
    
    if world.hit(r, 0.0, INFINITY, &mut rec) {
	return (rec.normal() + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction : Vec3 = r.direction().normalize();
    let t : f64 = 0.5 * (unit_direction.y() + 1.0);

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

    // World

    // Vec<Box<dyn Hittable>>
    let mut object_list : Vec<Box<dyn Hittable>> = Vec::new();

    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -100.0, -1.0), 100.0)));    
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    
    let world : Box<dyn Hittable> = Box::new(Hittable_List::new(object_list));

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Point3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    println!("P3");
    println!("{} {}" , image_width , image_height);
    println!("255");

    for i in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} " , i);
        for j in 0..image_width {
            let u: f64 = (j as f64) / ((image_width - 1) as f64);
            let v: f64 = (i as f64) / ((image_height - 1) as f64);
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color: Color = ray_color(r, &world);

            write_color(pixel_color)
        }
    }

    eprintln!("Done!");
}

/*
fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    println!("P3");
    println!("{} {}" , image_width , image_height);
    println!("255");

    for i in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} " , i);
        for j in 0..image_width {
            let r: f64 = (j as f64) / ((image_width as f64) - 1.0);
            let g: f64 = (i as f64) / ((image_height as f64) - 1.0);
            let b: f64 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}" , ir , ig , ib);
        }
    }

    eprintln!("Done!");
}
*/
