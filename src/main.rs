mod error;
mod image;

use crate::image::{Rgba, RgbaImage};

fn main() {
    let path = std::env::args().nth(1).unwrap();

    println!("output path: {}", path);

    let mut image = RgbaImage::new(512, 512);

    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, Rgba::rgb(x as u8, y as u8, 255));
        }
    }

    image.save(path).unwrap();

    println!("Hello, world!");
}
