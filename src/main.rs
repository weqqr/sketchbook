#![allow(dead_code)]

mod bvh;
mod color;
mod error;
mod geometry;
mod integrator;
mod image;
mod material;
mod math;

use crate::geometry::*;
use crate::image::*;
use crate::integrator::{Integrator, PrimaryRayIntegrator};
use crate::math::*;

fn main() {
    let path = if let Some(path) = std::env::args().nth(1) {
        path
    } else {
        eprintln!("usage: sketchbook <path/to/output.png>");
        return;
    };

    println!("output path: {}", path);

    let mut image = RgbaImage::new(1024, 512);

    let sphere = Sphere {
        center: Vector3::new(0.0, 0.0, 2.0),
        radius: 0.5,
    };

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let top_left = Vector3::new(-2.0, 1.0, 1.0);

    let integrator = PrimaryRayIntegrator::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let u = x as Float / image.width() as Float;
            let v = y as Float / image.height() as Float;

            let direction = Vector3::new(top_left.x + 4.0 * u, top_left.y - 2.0 * v, 1.0);
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };

            let color = integrator.integrate(&ray, &sphere);
            image.set_pixel(x, y, color.0.into());
        }
    }

    println!("saving");
    image.save(path).unwrap();

    println!("Hello, world!");
}
