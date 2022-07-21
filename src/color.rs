use crate::vec3::Vec3;

pub type Color = Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!("{} {} {}", (256.0 * clamp(r, 0.0, 0.999)) as i32, (256.0 * clamp(g, 0.0, 0.999)) as i32, (256.0 * clamp(b, 0.0, 0.999)) as i32);
}