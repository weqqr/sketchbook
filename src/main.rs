#![allow(unused_imports)]
#![allow(dead_code)]

mod accelerator;
mod color;
mod error;
mod image;
mod integrator;
mod material;
mod math;
mod scene;
mod shape;

use crate::accelerator::*;
use crate::image::*;
use crate::integrator::{Integrator, PrimaryRayIntegrator};
use crate::math::*;
use crate::scene::*;
use crate::shape::*;

fn build_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add(Sphere {
        center: Vector3::new(0.0, 0.0, 2.0),
        radius: 0.5,
    });
    scene.add(Sphere {
        center: Vector3::new(0.6, 0.5, 3.0),
        radius: 0.5,
    });
    scene.add(Sphere {
        center: Vector3::new(0.5, -0.3, 0.3),
        radius: 0.5,
    });

    scene
}

fn main() {
    let path = if let Some(path) = std::env::args().nth(1) {
        path
    } else {
        eprintln!("usage: sketchbook <path/to/output.png>");
        return;
    };

    println!("output path: {}", path);

    let mut image = RgbaImage::new(1024, 512);

    let origin = Vector3::new(0.0, 0.0, -2.0);
    let top_left = Vector3::new(-2.0, 1.0, 1.0);

    let integrator = PrimaryRayIntegrator::new();
    let scene = build_scene();
    let accel = LinearAccelerator::new(&scene);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let u = x as Float / image.width() as Float;
            let v = y as Float / image.height() as Float;

            let direction = Vector3::new(top_left.x + 4.0 * u, top_left.y - 2.0 * v, 1.0);
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };

            let color = integrator.integrate(&ray, &accel);
            image.set_pixel(x, y, color.0.into());
        }
    }

    println!("saving");
    image.save(path).unwrap();

    println!("Hello, world!");
}
