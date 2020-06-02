use crate::math::*;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

impl Vector3 {
	pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

	pub fn new(x: Float, y: Float, z: Float) -> Vector3 {
		Vector3 { x, y, z }
	}

	pub fn dot(&self, rhs: Vector3) -> Float {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn cross(&self, rhs: Vector3) -> Vector3 {
		Vector3 {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x,
		}
	}

	pub fn normalize(&self) -> Vector3 {
		let len = self.len();
		*self / len
	}

	pub fn len(&self) -> Float {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn min(&self, rhs: Vector3) -> Vector3 {
		Vector3 {
			x: self.x.min(rhs.x),
			y: self.y.min(rhs.y),
			z: self.z.min(rhs.z),
		}
	}

	pub fn max(&self, rhs: Vector3) -> Vector3 {
		Vector3 {
			x: self.x.max(rhs.x),
			y: self.y.max(rhs.y),
			z: self.z.max(rhs.z),
		}
	}
}

impl Add for Vector3 {
	type Output = Vector3;
	fn add(self, rhs: Vector3) -> Vector3 {
		Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl Sub for Vector3 {
	type Output = Vector3;
	fn sub(self, rhs: Vector3) -> Vector3 {
		Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl Mul<Float> for Vector3 {
	type Output = Vector3;
	fn mul(self, rhs: Float) -> Vector3 {
		Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl Div<Float> for Vector3 {
	type Output = Vector3;
	fn div(self, rhs: Float) -> Vector3 {
		let reciprocal = 1.0 / rhs;
		Vector3::new(self.x * reciprocal, self.y * reciprocal, self.z * reciprocal)
	}
}

impl Index<Axis> for Vector3 {
	type Output = Float;
	fn index(&self, axis: Axis) -> &Float {
		match axis {
			Axis::X => &self.x,
			Axis::Y => &self.y,
			Axis::Z => &self.z,
		}
	}
}

impl IndexMut<Axis> for Vector3 {
	fn index_mut(&mut self, axis: Axis) -> &mut Float {
		match axis {
			Axis::X => &mut self.x,
			Axis::Y => &mut self.y,
			Axis::Z => &mut self.z,
		}
	}
}
