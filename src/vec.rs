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
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }

    pub fn length(&self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
}

/// vector + vector
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: &self.x + &other.x, y: &self.y + &other.y, z: &self.z + &other.z }
    }
}

/// vector + scalar
impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: f32) -> Vec3 {
        Vec3 { x: &self.x + other, y: &self.y + other, z: &self.z + other }
    }
}

/// vector - vector
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: &self.x - &other.x, y: &self.y - &other.y, z: &self.z - &other.z }
    }
}

/// vector + scalar
impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: f32) -> Vec3 {
        Vec3 { x: &self.x - other, y: &self.y - other, z: &self.z - other }
    }
}

/// vector * vector
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: &self.x * &other.x, y: &self.y * &other.y, z: &self.z * &other.z }
    }
}

/// vector * scalar
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 { x: &self.x * other, y: &self.y * other, z: &self.z * other }
    }
}

/// scalar * vector
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        return other * self;
    }
}

/// vector / scalar
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f32) -> Vec3 {
        return (1. / other) * self;
    }
}

/// dot product
#[inline]
pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    return &v1.x * &v2.x + &v1.y * &v2.y + &v1.z * &v2.z;
}

/// cross product
#[inline]
pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 { 
        x: &v1.y * &v2.z - &v1.z * &v2.y, 
        y: &v1.z * &v2.x - &v1.x * &v2.z, 
        z: &v1.x * &v2.y - &v1.y * &v2.x 
    }
}

#[inline]
pub fn normalize(v: Vec3) -> Vec3 {
    return v / v.length();
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new() {
        let v: Vec3 = Vec3::new(0., 0., 0.);

        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }

    #[test]
    fn test_zero() {
        let v: Vec3 = Vec3::zero();

        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }

    #[test]
    fn test_add_v_v() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(2., 1., 0.);
        let sum: Vec3 = v1 + v2;

        assert_eq!(sum.x, 3.);
        assert_eq!(sum.y, 3.);
        assert_eq!(sum.z, 3.);
    }

    #[test]
    fn test_add_v_s() {
        let v: Vec3 = Vec3::new(1., 2., 3.);
        let s: f32 = 2.;
        let sum: Vec3 = v + s;

        assert_eq!(sum.x, 3.);
        assert_eq!(sum.y, 4.);
        assert_eq!(sum.z, 5.);
    }

    #[test]
    fn test_mul_v_v() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(2., 2., 0.);
        let mul: Vec3 = v1 * v2;

        assert_eq!(mul.x, 2.);
        assert_eq!(mul.y, 4.);
        assert_eq!(mul.z, 0.);
    }

    
    #[test]
    fn test_mul_s_v() {
        let v: Vec3 = Vec3::new(1., 2., 3.);
        let s: f32 = 2.;
        let mul: Vec3 = s * v;

        assert_eq!(mul.x, 2.);
        assert_eq!(mul.y, 4.);
        assert_eq!(mul.z, 6.);
    }

    #[test]
    fn test_mul_v_s() {
        let v: Vec3 = Vec3::new(1., 2., 3.);
        let s: f32 = 2.;
        let mul: Vec3 = v * s;

        assert_eq!(mul.x, 2.);
        assert_eq!(mul.y, 4.);
        assert_eq!(mul.z, 6.);
    }

    #[test]
    fn test_div_v_s() {
        let v: Vec3 = Vec3::new(4., 2., 6.);
        let s: f32 = 2.;
        let div: Vec3 = v / s;

        assert_eq!(div.x, 2.);
        assert_eq!(div.y, 1.);
        assert_eq!(div.z, 3.);
    }

    #[test]
    fn test_dot() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(1., 5., 7.);
        let d = dot(v1, v2);

        assert_eq!(d, 32.);
    }

    #[test]
    fn test_cross() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(1., 5., 7.);
        let c = cross(v1, v2);

        assert_eq!(c.x, -1.);
        assert_eq!(c.y, -4.);
        assert_eq!(c.z, 3.);
    }
    
    #[test]
    fn test_normalize() {
        let v: Vec3 = Vec3::new(2., 0., 0.);
        let n = normalize(v);
        assert_eq!(n.x, 1.);
        assert_eq!(n.y, 0.);
        assert_eq!(n.z, 0.);
    }
}