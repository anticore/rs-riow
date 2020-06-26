#![allow(dead_code)]

pub const PI: f32 = 3.1415926535897932385;
pub const INFINITY: f32 = f32::INFINITY;

pub fn deg_to_rad(degrees: f32) -> f32 {
    return degrees * PI / 180.;
}