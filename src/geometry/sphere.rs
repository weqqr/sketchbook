use crate::geometry::*;
use crate::math::*;

pub struct Sphere {
    pub center: Vector3,
    pub radius: Float,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let h = b * b - c;
        if h < 0.0 {
            return None;
        }

        let t = -b - h;
        if t > 0.0 {
            return Some(Hit {
                ray: ray.clone(),
                t,
                normal: Vector3::new(0.0, 0.0, 0.0),
            });
        }

        let t = -b + h;

        if t > 0.0 {
            return Some(Hit::new(ray: ray.clone(), t, normal: Vector3::new(0.0, 0.0, 0.0));
        }

        None
    }
}
