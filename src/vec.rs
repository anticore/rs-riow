#![allow(dead_code)]

use std::ops;
use std::primitive::f32;

/// 3d vector
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }

    pub fn length(&self) -> f32 {
        return f32::sqrt(self.length_squared());
    }
    
    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    /// dot product
    pub fn dot(v1: Vec3, v2: Vec3) -> f32 { 
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    /// cross product
    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 { 
            x: v1.y * v2.z - v1.z * v2.y, 
            y: v1.z * v2.x - v1.x * v2.z, 
            z: v1.x * v2.y - v1.y * v2.x 
        }
    }

    /// calculate unit vector
    pub fn normalize(self) -> Vec3 {
        return self / self.length();
    }
}

///-vector
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// vector + vector
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

/// vector + scalar
impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 { x: self.x + other, y: self.y + other, z: self.z + other }
    }
}

/// vector - vector
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - &other.z }
    }
}

/// vector - scalar
impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3 { x: self.x - other, y: self.y - other, z: self.z - other }
    }
}

/// vector * vector
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

/// vector * scalar
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

/// scalar * vector
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return other * self;
    }
}

/// vector / scalar
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        return (1. / other) * self;
    }
}
