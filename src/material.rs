use crate::color::Color;
use crate::math::*;

pub trait Material {
	fn albedo(&self) -> Color;
	fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color;
	fn emittance(&self) -> Color;
}

pub struct Lambertian {
	pub color: Color,
}

impl Material for Lambertian {
	fn albedo(&self) -> Color {
		self.color.clone()
	}

	fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color {
		self.color.clone() / PI
	}

	fn emittance(&self) -> Color {
		Color::new(0.0, 0.0, 0.0)
	}
}

pub struct LightEmitter {
	pub color: Color,
}

impl Material for LightEmitter {
	fn albedo(&self) -> Color {
		self.color
	}

	fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color {
		Color::new(0.0, 0.0, 0.0)
	}

	fn emittance(&self) -> Color {
		self.color
	}
}
