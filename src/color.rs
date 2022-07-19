use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = (255.999 * pixel_color.r()) as i32;
    let g = (255.999 * pixel_color.g()) as i32;
    let b = (255.999 * pixel_color.b()) as i32;

    println!("{} {} {}", r, g, b);
}