#![allow(dead_code)]

use crate::vec;
use crate::vec::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.
        }
    }
}

pub trait Hittable {
    fn hit(self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // solve t^2*b . b + 2*t*b . (A-C) + (A-C) . (A-C) - r^2 = 0 for t
        // t comes from the ray's equation origin + t * direction 
        // so t is the intersection of the ray with the sphere

        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = vec::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0. {
            let mut rec = HitRecord::new();

            let root = f32::sqrt(discriminant);
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(rec.t);
                rec.normal = (rec.point - self.center) / self.radius;
                return Some(rec);
            }

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(rec.t);
                rec.normal = (rec.point - self.center) / self.radius;
                return Some(rec);
            }
        } 
        return None;
    }
}