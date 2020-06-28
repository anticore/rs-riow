mod common;
use common::*;

mod vec;
use vec::Vec3;

mod image;
use image::Image;

mod ray;
use ray::Ray;

mod hittable;
use hittable::HittableList;
use hittable::Sphere;

mod camera;
use camera::Camera;

use std::time::Instant;

fn create_objects(world: &mut HittableList) {
    world.add(Sphere::new(Vec3::new(0., 0., -1.), 0.5).into());
    world.add(Sphere::new(Vec3::new(0., -100.5, -1.), 100.).into());
}

fn ray_color(ray: Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    match world.hit(ray, 0.001, 999.) {
        Some(r) => {
            let target: Vec3 = r.point + r.normal + Vec3::random_in_unit_sphere();
            return 0.5 * ray_color(Ray::new(r.point, target - r.point), &world, depth-1);
        }
        None => {
            let dir = ray.direction.normalize();
            let t = 0.5 * (dir.y + 1.);
            return (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

const IMAGE_WIDTH: u32 = 1280;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() {
    let now = Instant::now();
    
    let camera = Camera::new(16. / 9., 2., 1.);
    let image_height: u32 = (IMAGE_WIDTH as f32 / camera.aspect_ratio) as u32;

    let mut image = Image::new(IMAGE_WIDTH, image_height);

    let mut world = HittableList::new();
    create_objects(&mut world);
    

    for x in 0..IMAGE_WIDTH {
        print!(".");
        for y in 0..image_height {
            let mut color = Vec3::new(0., 0., 0.);

            for _ in 0..SAMPLES_PER_PIXEL {    
                let u: f32 = ((x as f32) + rnd!()) / (IMAGE_WIDTH as f32);
                let v: f32 = 1. - ((y as f32) + rnd!()) / (image_height as f32);

                let ray = camera.get_ray(u, v);
                color = color + ray_color(ray, &world, MAX_DEPTH); 
            }

            image.set_pixel(x, y, &color, SAMPLES_PER_PIXEL);
        }
    }

    image.save(String::from("image.png"));

    let elapsed = now.elapsed();

    println!("\n\tdone in {:.2?}", elapsed); 
}