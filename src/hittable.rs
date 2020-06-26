#![allow(dead_code)]

use crate::vec::Vec3;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum HittableObject {
    Sphere(Sphere)
}

impl HittableObject {
    pub fn hit(self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            HittableObject::Sphere(Sphere { center, radius }) => {
                // solve t^2*b . b + 2*t*b . (A-C) + (A-C) . (A-C) - r^2 = 0 for t
                // t comes from the ray's equation origin + t * direction 
                // so t is the intersection of the ray with the sphere

                let oc = ray.origin - center;
                let a = ray.direction.length_squared();
                let half_b = Vec3::dot(oc, ray.direction);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b*half_b - a*c;

                if discriminant > 0. {
                    let mut rec = HitRecord::new();

                    let root = f32::sqrt(discriminant);
                    let temp = (-half_b - root) / a;

                    if temp < t_max && temp > t_min {
                        rec.t = temp;
                        rec.point = ray.at(rec.t);
                        let outward_normal = (rec.point - center) / radius;
                        rec.set_face_normal(ray, outward_normal);
                        return Some(rec);
                    }

                    if temp < t_max && temp > t_min {
                        rec.t = temp;
                        rec.point = ray.at(rec.t);
                        let outward_normal = (rec.point - center) / radius;
                        rec.set_face_normal(ray, outward_normal);
                        return Some(rec);
                    }
                } 
                return None;
            }
        }
    }
}

impl From<Sphere> for HittableObject {
    fn from(v: Sphere) -> HittableObject {
        HittableObject::Sphere(v)
    }
}

#[derive(Debug, Clone)]
pub struct HittableList {
    v: Vec<HittableObject>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            v: vec![]
        }
    }

    pub fn add(&mut self, item: HittableObject) {
        self.v.push(item);
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for o in self.v.iter() {
            match o.hit(ray, t_min, closest_so_far) {
                Some(r) => {
                    hit_anything = true;
                    closest_so_far = r.t;
                    rec = r;
                },
                None => {}
            }
        }

        if hit_anything { return Some(rec) } else { return None };
    }
}
