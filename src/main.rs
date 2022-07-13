use std::num;
use std::io::{self, Write};

#[derive(Default, Debug, Copy, Clone)]
struct Vec3 {
    x : f64,
    y : f64,
    z : f64,
}

type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    fn new(x : f64, y: f64, z : f64) -> Vec3 {
        Vec3 {x:x, y:y, z:z}
    }

    fn length_squared(&self) -> f64 {
	self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
	self.length_squared().sqrt()
    }

    fn unit_vector(&self) -> Vec3 {
	*self / self.length()
    }
}

// --------------------------------------
// Operators overloading

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other : Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other : Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other : f64) -> Vec3 {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other : Vec3) -> f64 {
	self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other : f64) -> Vec3 {
	Vec3{x: self.x / other, y: self.y / other, z: self.z / other}
    }
}

// --------------------------------------

#[derive(Default, Debug)]
struct Ray {
    dir : Vec3,
    orig : Point3
}

impl Ray {
    fn at(&self, t : f64) -> Point3 {
	self.orig + self.dir * t
    }
}

// --------------------------------------

fn write_color(color : Color) {
    // io::stdout().write_all(b"hello world")?;
    let ir : u32 = (255.999 * color.x) as u32;
    let ig : u32 = (255.999 * color.y) as u32;
    let ib : u32 = (255.999 * color.z) as u32;	    
    println!("{} {} {}", ir, ig, ib);
}

fn ray_color(r : &Ray) -> Color {
    let unit_direction : Vec3 = Vec3::unit_vector(&r.dir);
    let t : f64 = 0.5 * unit_direction.y + 1.0;
    Color {x: 1.0, y:1.0, z: 1.0} * (1.0 - t) + Color {x: 0.5, y:0.7, z: 1.0} * t
}

fn main() {
    let default_v3 : Vec3 = Vec3{..Default::default()};
    let v3 : Vec3 = Vec3::new(1.0, 2.0, 3.0);

    // ----------------------

    // Image
    let aspect_ratio : f64 = 16.0 / 9.0;
    let image_width : u32 = 400;
    let image_height : u32 = ((image_width as f64) / aspect_ratio) as u32;

    // Camera
    let viewport_height : f64 = 2.0;
    let viewport_width : f64 = aspect_ratio * viewport_height;
    let focal_length : f64 = 1.0;

    let origin : Point3 = Point3{..Default::default()};
    let horizontal = Vec3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x:0.0, y: viewport_height, z:0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x:0.0, y:0.0, z:focal_length};
    
    println!("P3\n{} {} \n255", image_width, image_height);
    for j in (0..image_height).rev() {
	eprintln!("\rScanlines remaining: {} ", j);
	for i in 0..image_width {
	    let u : f64 = i as f64 / (image_width - 1) as f64;
	    let v : f64 = j as f64 / (image_height - 1) as f64;

	    let r : Ray = Ray {orig:origin, dir:lower_left_corner + horizontal * u + vertical * v - origin};
	    let pixel_color : Color = ray_color(&r);
            write_color(pixel_color);
	}
    }
    eprintln!("\nDone.\n");

}
