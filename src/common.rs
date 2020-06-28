#![allow(dead_code)]
#![macro_use]

use rand::Rng;

pub const PI: f32 = 3.1415926535897932385;
pub const INFINITY: f32 = f32::INFINITY;

pub fn deg_to_rad(degrees: f32) -> f32 {
    return degrees * PI / 180.;
}

pub fn random_f(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max);
}

#[macro_export]
macro_rules! rnd {
    ($a: literal, $b: literal) => {
        random_f($a, $b)
    };
    () => {
        random_f(0., 1.)
    }
}

pub fn clamp(n: f32, min: f32, max: f32) -> f32 {
    if n < min { return min } 
    if n > max { return max }
    return n;
}