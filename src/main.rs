mod vec;
mod color;
use color::Color;
mod image;
use image::Image;
mod ray;

fn main() {
    let mut image = Image::new(320, 240);

    for x in 0..320 {
        for y in 0..240 {
            let color = Color::new((x as f64) / 256., (y as f64) / 256., 0.4, 1.);
            image.set_pixel(x, y, &color);
        }
    }

    image.save(String::from("image.png"));
}