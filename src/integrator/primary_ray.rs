use crate::color::Color;
use crate::shape::Shape;
use crate::integrator::Integrator;
use crate::math::*;
use crate::accelerator::Accelerator;

pub struct PrimaryRayIntegrator;

impl PrimaryRayIntegrator {
	pub fn new() -> Self {
		Self
	}
}

impl Integrator for PrimaryRayIntegrator {
	type Output = (Color, Vector3);
	fn integrate<A: Accelerator>(&self, ray: &Ray, accel: &A) -> (Color, Vector3) {
		let hit = accel.trace(ray);
		if let None = hit {
			return (Color::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0));
		}

		let albedo = Color {
			r: 1.0,
			g: 0.0,
			b: 0.0,
		};

		let normal = Vector3 {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		};

		(albedo, normal)
	}
}
