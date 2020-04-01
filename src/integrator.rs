pub mod path_tracer;
pub mod primary_ray;

pub use path_tracer::*;
pub use primary_ray::*;

use crate::shape::Shape;
use crate::math::Ray;
use crate::accelerator::Accelerator;

pub trait Integrator {
	type Output;
	fn integrate<A: Accelerator>(&self, ray: &Ray, accel: &A) -> Self::Output;
}
