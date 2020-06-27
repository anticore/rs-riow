#![allow(dead_code)]

use crate::vec::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32,  viewport_height: f32, focal_length: f32) -> Camera {
        let viewport_width: f32 = aspect_ratio * viewport_height;
        
        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin, 
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        );
    }
}