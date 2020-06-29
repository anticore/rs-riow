#![allow(dead_code)]

use crate::common::*;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec::Vec3;


pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1. - r0) * f32::powi(1. - cosine, 5);
}


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
pub struct DielectricMaterial {
    pub ref_idx: f32
}

impl DielectricMaterial {
    pub fn new(ref_idx: f32) -> DielectricMaterial {
        DielectricMaterial {
            ref_idx
        }
    }
}

impl From<DielectricMaterial> for Material {
    fn from(v: DielectricMaterial) -> Material {
        Material::DielectricMaterial(v)
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Material {
    LambertianMaterial(LambertianMaterial),
    MetalMaterial(MetalMaterial),
    DielectricMaterial(DielectricMaterial)
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
            },

            Material::DielectricMaterial(DielectricMaterial { ref_idx }) => {
                let attenuation = Vec3::new(1., 1., 1.);
                let etai_over_etat = if rec.front_face { 1.0 / ref_idx } else { ref_idx };

                let direction = ray.direction.normalize();

                let cos_theta = f32::min(Vec3::dot(-direction, rec.normal), 1.);
                let sin_theta = f32::sqrt(1. - cos_theta * cos_theta);

                if etai_over_etat * sin_theta > 1. {
                    let reflected = direction.reflect(rec.normal);
                    let scattered = Ray::new(rec.point, reflected);
                    return (true, scattered, attenuation);
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if random_f(0., 1.) < reflect_prob {
                    let reflected = direction.reflect(rec.normal);
                    let scattered = Ray::new(rec.point, reflected);
                    return (true, scattered, attenuation);
                }

                let refracted = direction.refract(rec.normal, etai_over_etat);
                let scattered = Ray::new(rec.point, refracted);
                return (true, scattered, attenuation);
            }
        }
    }
}

