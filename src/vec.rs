#![allow(dead_code)]

use std::ops;

/// 3d vector
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

    #[inline]
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

/// vector + scalar
impl ops::Add<f64> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: f64) -> Vec3 {
        Vec3 { x: self.x + other, y: self.y + other, z: self.z + other }
    }
}

/// vector * vector
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

/// vector * scalar
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f64) -> Vec3 {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

/// scalar * vector
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 { x: self * other.x, y: self * other.y, z: self * other.z }
    }
}

/// vector / scalar
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f64) -> Vec3 {
        return (1. / other) * self;
    }
}

/// dot product
#[inline]
pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
}


/// cross product
#[inline]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 { 
        x: v1.y * v2.z - v1.z * v2.y, 
        y: v1.z * v2.x - v1.x * v2.z, 
        z: v1.x * v2.y - v1.y * v2.x 
    }
}