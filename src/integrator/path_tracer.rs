use crate::color::Color;
use crate::integrator::Integrator;
use crate::math::*;
/*
pub struct PathTracer {
	bounces: usize,
}

impl PathTracer {
	pub fn new(bounces: usize) -> PathTracer {
		PathTracer {
			bounces,
		}
	}

	pub fn trace(&self) {
	}
}

fn lambertian(wi: Vector3, wo: Vector3) -> Color {
	Color::new(1.0, 1.0, 1.0) / PI
}

impl Integrator for PathTracer {
	type Output = Color;
	fn integrate<S: Hitabl>(&self, ray: &Ray, scene: S) -> Color {
		let hit = scene.hit(ray);
		if let None = hit {
			return Color::new(1.0, 1.0, 1.0);
		}

		let wi = ray.direction;
		let wo = hit.point + hit.normal + Vector3::random_sphere();

		let next_color = self.trace();
		let brdf = lambertian(wi, wo) * next_color;

		Color {
			r: 1.0,
			g: 0.0,
			b: 0.0,
		}
	}
}
*/
