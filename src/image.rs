use crate::error::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn rgb(r: u8, g: u8, b: u8) -> Rgba {
        Rgba { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }
}

pub struct RgbaImage {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl RgbaImage {
    const DEPTH: usize = 4;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; Self::DEPTH * width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn pixel_offset(&self, x: usize, y: usize) -> usize {
        Self::DEPTH * (y * self.width + x)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Rgba {
        let offset = self.pixel_offset(x, y);

        Rgba {
            r: self.data[offset + 0],
            g: self.data[offset + 1],
            b: self.data[offset + 2],
            a: self.data[offset + 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgba) {
        let offset = self.pixel_offset(x, y);

        self.data[offset + 0] = color.r;
        self.data[offset + 1] = color.g;
        self.data[offset + 2] = color.b;
        self.data[offset + 3] = color.a;
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(&mut buf_writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;

        Ok(())
    }
}
