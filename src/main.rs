mod vec;
mod color;
use color::Color;
mod image;
use image::Image;

fn main() {
    let mut image = Image::new(256, 256);

    for x in 0..256 {
        for y in 0..256 {
            let color = Color::new((x as f64) / 256., (y as f64) / 256., 0.4, 1.);
            image.set_pixel(x, y, &color);
        }
    }

    image.save(String::from("image.png"));
}