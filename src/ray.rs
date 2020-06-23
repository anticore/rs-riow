#![allow(dead_code)]

use crate::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction }
    }

    pub fn at(self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new() {
        let origin = Vec3::new(0., 0., 0.);
        let direction = Vec3::new(1., 1., 1.);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin.x, 0.);
    }

    #[test]
    fn test_at() {
        let origin = Vec3::new(0., 0., 0.);
        let direction = Vec3::new(1., 1., 1.);
        let ray = Ray::new(origin, direction);

        let at = ray.at(2.);
        assert_eq!(at.x, 2.);
        assert_eq!(at.y, 2.);
        assert_eq!(at.z, 2.);
    }
}