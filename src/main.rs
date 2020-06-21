#![allow(unused_imports)]
#![allow(dead_code)]

mod accelerator;
mod color;
mod error;
mod image;
mod integrator;
mod io;
mod material;
mod math;
mod random;
mod scene;
mod shape;

use crate::accelerator::*;
use crate::color::*;
use crate::image::*;
use crate::integrator::*;
use crate::material::*;
use crate::math::*;
use crate::random::*;
use crate::scene::*;
use crate::shape::*;
use std::time::Instant;

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    let red = scene.add_material(Lambertian {
        color: Color::new(1.0, 0.0, 0.0),
    });

    let green = scene.add_material(Lambertian {
        color: Color::new(0.0, 1.0, 0.0),
    });

    let grey = scene.add_material(Lambertian {
        color: Color::new(1.0, 1.0, 1.0),
    });

    let yellow = scene.add_material(Lambertian {
        color: Color::new(0.9, 0.7, 0.1),
    });

    let light = scene.add_material(LightEmitter {
        color: Color::new(10.0, 10.0, 10.0),
    });

    scene.add_shape(Sphere {
        center: Vector3::new(0.5, -0.2, -0.5),
        radius: 0.3,
        material: green,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(0.0, 0.0, 3.0),
        radius: 0.5,
        material: red,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(-1.5, 0.0, 0.3),
        radius: 0.5,
        material: light,
    });
    scene.add_shape(Triangle {
        a: Vector3::new(-1.0, 0.5, 0.5),
        b: Vector3::new(-2.0, 1.3, 0.0),
        c: Vector3::new(-1.8, 0.8, 0.3),
        na: Vector3::new(0.0, 1.0, 0.0),
        nb: Vector3::new(0.0, 1.0, 0.0),
        nc: Vector3::new(0.0, 1.0, 0.0),
        material: red,
    });
    scene.add_shape(crate::shape::plane::Plane {
        point: Vector3::new(0.0, -0.5, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        material: grey,
    });

    scene.add_shape(crate::shape::plane::Plane {
        point: Vector3::new(-2.0, -0.5, 0.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        material: yellow,
    });

    scene
}

pub struct Stats {
    pub ray_count: u64,
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

    let mut integrator = PathTracer::new(4);
    // let mut integrator = PrimaryRayIntegrator::new();
    let scene = build_scene();
    let accel = LinearAccelerator::new(&scene);

    let sample_count = 64;

    let mut stats = Stats {
        ray_count: 0,
    };

    let mut rng = RandomGenerator::new();

    let start = Instant::now();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let width = image.width() as Float;
            let height = image.height() as Float;

            let u = x as Float / width;
            let v = y as Float / height;

            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..sample_count {
                let u = u + rng.unit() / width;
                let v = v + rng.unit() / height;

                let direction = Vector3::new(top_left.x + 4.0 * u, top_left.y - 2.0 * v, 1.0);
                let ray = Ray {
                    origin,
                    direction: direction.normalize(),
                };

                color = color + integrator.integrate(&scene, &ray, &accel, &mut stats).0;
            }
            image.set_pixel(x, y, (color / sample_count as f64).into());
        }
        println!("y={}, {} rays", y, stats.ray_count);
    }

    let elapsed = Instant::now().duration_since(start).as_secs_f64();
    let mrays_per_second = (stats.ray_count as f64) / elapsed / 1000000.0;
    println!("{} MRays per second", mrays_per_second);

    println!("saving");
    image.save(path).unwrap();

    println!("Hello, world!");
}
