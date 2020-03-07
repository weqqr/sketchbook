pub mod path_tracer;
pub mod primary_ray;

pub use path_tracer::*;
pub use primary_ray::*;

use crate::color::Color;
use crate::geometry::{Ray, Object};

pub trait Integrator {
	type Output;
	fn integrate<S: Object>(&self, ray: &Ray, scene: &S) -> Self::Output;
}
