use crate::vec3::{Vec3, Point3, Color};

#[derive(Debug, Copy, Clone)]
pub enum Texture {
    Checker { even: Color, odd: Color },
}

impl Texture {
    pub fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        match self {
            Self::Checker { even, odd } => {
                let sines: f64 = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
                if sines < 0.0 { *odd } else { *even }
            }
        }
    }

    pub fn new_checker(even: &Color, odd: &Color) -> Self {
        Self::Checker { even: *even, odd: *odd }
    }
}