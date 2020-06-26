mod vec;
use vec::Vec3;

mod color;
use color::Color;

mod image;
use image::Image;

mod ray;
use ray::Ray;

mod hittable;
use hittable::HittableList;
use hittable::Sphere;

mod common;
#[allow(unused_imports)]
use common::*;

const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const VIEWPORT_HEIGHT: f32 = 2.;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.;

fn print_info() {
    println!();
    println!("Aspect Ratio = {}\nImage Size = {} x {}", ASPECT_RATIO, IMAGE_WIDTH, IMAGE_HEIGHT);
    println!();
}

fn create_objects(world: &mut HittableList) {
    world.add(Sphere::new(Vec3::new(0., 0., -1.), 0.5).into());
    world.add(Sphere::new(Vec3::new(0., -100.5, -1.), 100.).into());
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    match world.hit(ray, 0., 99999.) {
        Some(r) => {
            return Color::new(
                0.5 * (r.normal.x + 1.),
                0.5 * (r.normal.y + 1.),
                0.5 * (r.normal.z + 1.),
                1.
            )
        }
        None => {
            let dir = ray.direction.normalize();
            let t = 0.5 * (dir.y + 1.);
            return (1. - t) * Color::new(1., 1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0, 1.0);
        }
    }
}


fn main() {
    print_info();

    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin: Vec3 = Vec3::zero();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new( 0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner = origin - (horizontal / 2.) - (vertical / 2.) - Vec3::new(0., 0., FOCAL_LENGTH);

    let mut world = HittableList::new();
    create_objects(&mut world);
    

    for x in 0..IMAGE_WIDTH {
        print!(".");
        for y in 0..IMAGE_HEIGHT {
            let u: f32 = (x as f32) / (IMAGE_WIDTH as f32);
            let v: f32 = 1. - (y as f32) / (IMAGE_HEIGHT as f32);

            let direction: Vec3 = lower_left_corner + (u * horizontal) + (v * vertical) - origin;

            let ray: Ray = Ray::new(origin, direction);
            let color: Color = ray_color(ray, &world);
            image.set_pixel(x, y, &color);
        }
    }

    image.save(String::from("image.png"));

    println!("done");
}