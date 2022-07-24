use rand::prelude::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128Plus;

use crate::vec3::{Vec3, Point3, Color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::material::Material;
use crate::moving_sphere::MovingSphere;
use crate::texture::Texture;
use crate::camera::Camera;


// Random scene from the end of book 1 (+ bounce and checkered ground)
pub fn random_scene(bounce: bool) -> HittableList {
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();

    //let ground_material: Material = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    let ground_material: Material = Material::new_textured(Texture::new_checker(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut rng = Xoroshiro128Plus::from_entropy();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center: Point3 = Point3::new((a as f64) + 0.9 * rng.gen::<f64>(), 0.2, (b as f64) + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Color = Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
                    let sphere_material: Material = Material::new_lambertian(albedo);
                    if bounce {
                        let center2: Point3 = center + Point3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                        object_list.push(Box::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material)));
                    } else {
                        object_list.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
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

    let material2 : Material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    object_list.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 : Material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    object_list.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let world = HittableList::new(object_list);
    return world;
}

pub fn random_scene_cam(aspect_ratio: f64) -> Camera {
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0)
}

// ----

// Two checkered spheres
pub fn two_spheres() -> HittableList {
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();
    let checker: Material = Material::new_textured(Texture::new_checker(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, checker)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, checker)));

    let world = HittableList::new(object_list);
    return world;
}

pub fn two_spheres_cam(aspect_ratio: f64) -> Camera {
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.0;

    Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0)
}

// ---
// Two spheres, one is checkered, one is a light
pub fn light_test() -> HittableList {
    let mut object_list: Vec<Box<dyn Hittable>> = Vec::new();
    let checker: Material = Material::new_textured(Texture::new_checker(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let light: Material = Material::new_light(Color::new(1.0, 1.0, 1.0));

    object_list.push(Box::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, checker)));
    object_list.push(Box::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 8.0, light)));

    let world = HittableList::new(object_list);
    return world;
}