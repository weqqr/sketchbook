pub mod path_tracer;

pub use path_tracer::*;

use crate::color::Color;
use crate::geometry::Ray;

pub struct IntegrationParameters {

}

pub trait Integrator {
	fn integrate(&self, ray: &Ray) -> Color;
}
