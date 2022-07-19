use crate::vec3::Vec3;

pub type Color = Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.r();
    let mut g = pixel_color.g();
    let mut b = pixel_color.b();

    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    r = 256.0 * clamp(r * scale, 0.0, 0.999);
    g = 256.0 * clamp(g * scale, 0.0, 0.999);
    b = 256.0 * clamp(b * scale, 0.0, 0.999);

    println!("{} {} {}", r as i32, g as i32, b as i32);
}