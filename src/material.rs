#![allow(dead_code)]

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec::Vec3;



#[derive(Debug, Copy, Clone)]
pub struct LambertianMaterial {
    pub albedo: Vec3
}

impl LambertianMaterial {
    pub fn new(albedo: Vec3) -> LambertianMaterial {
        LambertianMaterial {
            albedo
        }
    }
}

impl From<LambertianMaterial> for Material {
    fn from(v: LambertianMaterial) -> Material {
        Material::LambertianMaterial(v)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzz: f32
}

impl MetalMaterial {
    pub fn new(albedo: Vec3, fuzz: f32) -> MetalMaterial {
        let mut f = fuzz;
        if fuzz > 1. { f = 1. };
        MetalMaterial {
            albedo,
            fuzz: f
        }
    }
}

impl From<MetalMaterial> for Material {
    fn from(v: MetalMaterial) -> Material {
        Material::MetalMaterial(v)
    }
}



#[derive(Debug, Copy, Clone)]
pub enum Material {
    LambertianMaterial(LambertianMaterial),
    MetalMaterial(MetalMaterial)
}

impl Material {
    pub fn scatter(self, ray: Ray, rec: HitRecord) -> (bool, Ray, Vec3) {
        match self {
            Material::LambertianMaterial(LambertianMaterial { albedo }) => {
                let scatter_direction = rec.normal + Vec3::random_unit_vector();
                let scattered = Ray::new(rec.point, scatter_direction);
                let attenuation = albedo;
        
                return (true, scattered, attenuation)
            },

            Material::MetalMaterial(MetalMaterial { albedo, fuzz }) => {
                let reflected = ray.direction.normalize().reflect(rec.normal);
                let scattered = Ray::new(rec.point, reflected + fuzz * Vec3::random_in_unit_sphere());
                let attenuation = albedo;
        
                return (Vec3::dot(scattered.direction, rec.normal) > 0., scattered, attenuation)
            }
        }
    }
}

