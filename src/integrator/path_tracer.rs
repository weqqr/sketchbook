use crate::accelerator::*;
use crate::color::Color;
use crate::integrator::Integrator;
use crate::math::*;
use crate::random::*;
use crate::scene::*;

pub struct PathTracer {
    bounces: usize,
    rng: RandomGenerator,
    pub ray_count: usize,
}

impl PathTracer {
    pub fn new(bounces: usize) -> PathTracer {
        PathTracer {
            bounces,
            rng: RandomGenerator::new(),
            ray_count: 0,
        }
    }

    pub fn trace<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A, bounce: usize) -> (Color, Vector3) {
        if bounce >= self.bounces {
            return (Color::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        }
        self.ray_count += 1;
        let hit = accel.trace(ray);
        let hit = if let Some(hit) = hit {
            hit
        } else {
            return (scene.world_color, Vector3::new(0.0, 0.0, 0.0));
        };

        let point = hit.point();
        let normal = hit.shape.normal_at(point);
        let material = scene.get_material(hit.shape.material());

        let next_ray = Ray {
            origin: point,
            direction: (normal + Vector3::random_point_on_unit_sphere(&mut self.rng)).normalize(),
        };

        let next_color = self.trace(scene, &next_ray, accel, bounce + 1).0;

        let wi = ray.direction;
        let wo = next_ray.direction;
        let color = material.emittance() + material.brdf(normal, wi, wo) * next_color;

        (color, normal)
    }
}

impl Integrator for PathTracer {
    type Output = (Color, Vector3);
    fn integrate<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A) -> (Color, Vector3) {
        self.trace(scene, ray, accel, 0)
    }
}
