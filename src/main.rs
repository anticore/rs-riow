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

fn create_objects(world: &mut HittableList) {
    world.add(Sphere::new(Vec3::new(0., 0., -1.), 0.5).into());
    world.add(Sphere::new(Vec3::new(0., -100.5, -1.), 100.).into());
}

fn ray_color(ray: Ray, world: &HittableList) -> Vec3 {
    match world.hit(ray, 0., 99999.) {
        Some(r) => {
            return Vec3::new(
                0.5 * (r.normal.x + 1.),
                0.5 * (r.normal.y + 1.),
                0.5 * (r.normal.z + 1.)
            )
        }
        None => {
            let dir = ray.direction.normalize();
            let t = 0.5 * (dir.y + 1.);
            return (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}


fn main() {
    let camera = Camera::new(16. / 9., 2., 1.);
    let image_width: u32 = 384;
    let image_height: u32 = (image_width as f32 / camera.aspect_ratio) as u32;

    let mut image = Image::new(image_width, image_height);

    let mut world = HittableList::new();
    create_objects(&mut world);
    
    let samples_per_pixel = 10;

    for x in 0..image_width {
        for y in 0..image_height {
            print!(".");
            let mut color = Vec3::new(0., 0., 0.);

            for _ in 0..samples_per_pixel {    
                let u: f32 = ((x as f32) + rnd!()) / (image_width as f32);
                let v: f32 = 1. - ((y as f32) + rnd!()) / (image_height as f32);

                let ray = camera.get_ray(u, v);
                color = color + ray_color(ray, &world);
            }

            image.set_pixel(x, y, &color, samples_per_pixel);
        }
    }

    image.save(String::from("image.png"));

    println!(" done");
}