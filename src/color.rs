use crate::math::*;
use crate::image::Rgba;
use std::ops::{Add, Sub, Mul, Div};

const GAMMA: Float = 2.2;

#[derive(Clone, Copy)]
pub struct Color {
	pub r: Float,
	pub g: Float,
	pub b: Float,
}

impl Color {
	pub fn new(r: Float, g: Float, b: Float) -> Self {
		Color { r, g, b }
	}
}

fn gamma(component: Float) -> Float {
	component.powf(2.2)
}

fn degamma(component: Float) -> Float {
	component.powf(1.0 / 2.2)
}

impl From<Rgba> for Color {
	fn from(c: Rgba) -> Color {
		Color {
			r: gamma(c.r as Float / 255.0),
			g: gamma(c.g as Float / 255.0),
			b: gamma(c.b as Float / 255.0),
		}
	}
}

impl Into<Rgba> for Color {
	fn into(self) -> Rgba {
		Rgba {
			r: (degamma(clamp(self.r, 0.0, 1.0)) * 255.0) as u8,
			g: (degamma(clamp(self.g, 0.0, 1.0)) * 255.0) as u8,
			b: (degamma(clamp(self.b, 0.0, 1.0)) * 255.0) as u8,
			a: 255,
		}
	}
}

impl Add for Color {
	type Output = Self;
	fn add(self, rhs: Color) -> Color {
		Color {
			r: self.r + rhs.r,
			g: self.g + rhs.g,
			b: self.b + rhs.b,
		}
	}
}

impl Sub for Color {
	type Output = Self;
	fn sub(self, rhs: Color) -> Color {
		Color {
			r: self.r - rhs.r,
			g: self.g - rhs.g,
			b: self.b - rhs.b,
		}
	}
}

impl Mul for Color {
	type Output = Self;
	fn mul(self, rhs: Color) -> Color {
		Color {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
		}
	}
}

impl Div for Color {
	type Output = Self;
	fn div(self, rhs: Color) -> Color {
		Color {
			r: self.r / rhs.r,
			g: self.g / rhs.g,
			b: self.b / rhs.b,
		}
	}
}

impl Mul<Float> for Color {
	type Output = Self;
	fn mul(self, rhs: Float) -> Color {
		Color {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
		}
	}
}

impl Div<Float> for Color {
	type Output = Self;
	fn div(self, rhs: Float) -> Color {
		let reciprocal = 1.0 / rhs;
		Color {
			r: self.r * reciprocal,
			g: self.g * reciprocal,
			b: self.b * reciprocal,
		}
	}
}
