#![allow(dead_code)]

use std::ops;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }
}

/// vector + vector
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

/// vector + scalar
impl ops::Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3 { x: self.x + other, y: self.y + other, z: self.z + other }
    }
}