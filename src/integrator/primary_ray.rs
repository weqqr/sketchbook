use crate::accelerator::Accelerator;
use crate::color::Color;
use crate::integrator::Integrator;
use crate::math::*;
use crate::scene::Scene;
use crate::shape::Shape;

pub struct PrimaryRayIntegrator;

impl PrimaryRayIntegrator {
    pub fn new() -> Self {
        Self
    }
}

impl Integrator for PrimaryRayIntegrator {
    type Output = (Color, Vector3);
    fn integrate<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A) -> (Color, Vector3) {
        let hit = accel.trace(ray);
        let hit = if let Some(hit) = hit {
            hit
        } else {
            return (scene.world_color, Vector3::new(-1.0, -1.0, -1.0));
        };

        let albedo = scene.get_material(hit.shape.material()).albedo();
        let normal = hit.shape.normal_at(hit.point());

        (albedo, normal)
    }
}
