use crate::color::Color;
use crate::geometry::Ray;
use crate::integrator::Integrator;

pub struct PrimaryRayIntegrator {
	bounces: usize,
}

impl PrimaryRayIntegrator {
	pub fn new() -> Self {
		Self {
		}
	}
}

impl Integrator for PathTracer {
	fn integrate(&self, ray: &Ray) -> Color {
		let hit = scene.hit(ray);
		if let None = hit {
			return Color::new(1.0, 1.0, 1.0);
		}

		let wi = ray.direction;
		let wo = hit.point + hit.normal + Vector3::random_unit();

		let next_color = self.trace();
		let brdf = lambertian(wi, wo, hit.normal) * next_color;

		Color {
			r: 1.0,
			g: 0.0,
			b: 0.0,
		}
	}
}
