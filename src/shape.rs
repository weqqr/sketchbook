pub mod triangle;
pub mod sphere;

pub use self::triangle::*;
pub use self::sphere::*;

use crate::math::*;
use crate::material::Material;

pub trait Shape {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn normal_at(&self, point: Vector3) -> Vector3;
    fn material_at(&self, point: Vector3) -> &dyn Material;
    fn bounding_box(&self) -> Aabb;
    fn surface_area(&self) -> Float;
}

pub struct Hit<'shapes> {
    pub ray: Ray,
    pub t: Float,
    pub shape: &'shapes dyn Shape,
}

impl<'shapes> Hit<'shapes> {
    pub fn new(ray: &Ray, t: Float, shape: &'shapes dyn Shape) -> Hit<'shapes> {
        Hit {
            ray: ray.clone(),
            t,
            shape,
        }
    }
}
