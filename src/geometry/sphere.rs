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
            return Some(Hit::new(ray, t, self));
        }

        let t = -b + h;

        if t > 0.0 {
            return Some(Hit::new(ray, t, self));
        }

        None
    }

    fn normal_at(&self, point: Vector3) -> Vector3 {
        (point - self.center).normalize()
    }
}
