pub mod sphere;

pub use self::sphere::*;

use crate::math::*;

pub trait Shape {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn normal_at(&self, point: Vector3) -> Vector3;
    fn bounding_box(&self) -> Aabb;
    fn surface_area(&self) -> Float;
}

pub struct Hit<'o> {
    pub ray: Ray,
    pub t: Float,
    pub shape: &'o dyn Shape,
}

impl<'o> Hit<'o> {
    pub fn new(ray: &Ray, t: Float, shape: &'o dyn Shape) -> Hit<'o> {
        Hit {
            ray: ray.clone(),
            t,
            shape,
        }
    }
}
