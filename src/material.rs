use crate::color::Color;
use crate::math::*;

pub trait Material {
	fn albedo(&self) -> Color;
	fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color;
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
}
