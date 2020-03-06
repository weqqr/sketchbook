use crate::math::*;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3 {
    pub fn new(x: Float, y: Float, z: Float) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, rhs: Vector3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn normalize(&self) -> Vector3 {
		let len = self.len();
		*self / len
	}

	pub fn len(&self) -> Float {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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