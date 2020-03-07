pub mod sphere;

pub use self::sphere::*;

use crate::math::*;

pub trait Object {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn normal_at(&self, point: Vector3) -> Vector3;
}

pub struct Hit<'o> {
    ray: Ray,
    t: Float,
    object: &'o dyn Object,
}

impl<'o> Hit<'o> {
    pub fn new(ray: &Ray, t: Float, object: &'o dyn Object) -> Hit<'o> {
        Hit {
            ray: ray.clone(),
            t,
            object,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
	pub fn point_at(&self, t: Float) -> Vector3 {
		self.origin + self.direction * t
	}
}
