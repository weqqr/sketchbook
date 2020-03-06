use crate::math::*;

pub trait Material {
	fn albedo(&self) -> RgbaF;
}
