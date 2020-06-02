pub mod path_tracer;
pub mod primary_ray;

pub use path_tracer::*;
pub use primary_ray::*;

use crate::accelerator::Accelerator;
use crate::math::Ray;
use crate::scene::Scene;
use crate::shape::Shape;

pub trait Integrator {
	type Output;
	fn integrate<A: Accelerator>(&self, scene: &Scene, ray: &Ray, accel: &A) -> Self::Output;
}
