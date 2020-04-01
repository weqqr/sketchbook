use crate::shape::{Shape, Hit};
use crate::math::Ray;

pub mod kd;
pub mod linear;

pub use self::linear::LinearAccelerator;

pub trait Accelerator {
    fn trace(&self, ray: &Ray) -> Option<Hit>;
}