use crate::color::Color;
use crate::geometry::{Ray, Object};
use crate::integrator::Integrator;
use crate::math::*;

pub struct PrimaryRayIntegrator;

impl PrimaryRayIntegrator {
	pub fn new() -> Self {
		Self
	}
}

impl Integrator for PrimaryRayIntegrator {
	type Output = (Color, Vector3);
	fn integrate<S: Object>(&self, ray: &Ray, scene: &S) -> (Color, Vector3) {
		let hit = scene.hit(ray);
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
