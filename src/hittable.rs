#![allow(dead_code)]

use crate::vec::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
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
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0. {
            let mut rec = HitRecord::new();

            let root = f32::sqrt(discriminant);
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(rec.t);
                let outward_normal = (rec.point - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                return Some(rec);
            }

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(rec.t);
                let outward_normal = (rec.point - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                return Some(rec);
            }
        } 
        return None;
    }
}
