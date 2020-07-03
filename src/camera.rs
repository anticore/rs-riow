#![allow(dead_code)]

use crate::vec::Vec3;
use crate::ray::Ray;
use crate::common::*;

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,

    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,  
        vfov: f32,
        lookfrom: Vec3, 
        lookat: Vec3, 
        vup: Vec3
    ) -> Camera {
        let theta = deg_to_rad(vfov);
        let h = f32::tan(theta / 2.);
        let viewport_height = 2. * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = Vec3::cross(vup, w).normalize();
        let v = Vec3::cross(w, u);
        
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - w;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
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