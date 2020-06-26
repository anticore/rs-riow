#![allow(dead_code)]

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png;
use png::HasParameters;
use std::convert::TryFrom;
use crate::color::Color;


pub struct Image {
    pub buffer: Vec<u8>,

    pub width: u32,
    pub height: u32
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let length: usize = usize::try_from(width * 4 * height).unwrap();

        Image {
            width: width,
            height: height,
            buffer: vec![0; length]
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        let buffer_pos: usize = usize::try_from(y * self.width * 4 + x * 4).unwrap();


        self.buffer[buffer_pos] = (color.r * 255.).round() as u8;
        self.buffer[buffer_pos + 1] = (color.g * 255.).round() as u8;
        self.buffer[buffer_pos + 2] = (color.b * 255.).round() as u8;
        self.buffer[buffer_pos + 3] = (color.a * 255.).round() as u8;
    }

    pub fn save(self, path: String) {
        let path = Path::new(&path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width.into(), self.height.into());
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let data: &[u8] = self.buffer.as_slice();
        writer.write_image_data(data).unwrap();
    }
}
