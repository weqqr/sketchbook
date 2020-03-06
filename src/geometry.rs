pub mod sphere;

pub use self::sphere::*;

use crate::math::*;

pub trait Object {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Hit {
    ray: Ray,
    t: Float,
    point: Vector3,
    normal: Vector3,
}

impl Hit {
    pub fn new(ray: &Ray, t: Float, normal: Vector3) -> Hit {
        Hit {
            ray: ray.clone(),
            t,
            point: ray.point_at(t),
            normal:
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
