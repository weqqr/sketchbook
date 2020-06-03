use crate::math::Float;
use rand_pcg::Pcg64Mcg;
use rand_distr::StandardNormal;
use rand::Rng;

pub struct RandomGenerator {
    generator: Pcg64Mcg,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            generator: Pcg64Mcg::new(0xcafef00dd15ea5e5)
        }
    }

    pub fn next_gaussian(&mut self) -> Float {
        self.generator.sample(StandardNormal)
    }

    pub fn unit(&mut self) -> Float {
        self.generator.gen_range(0.0, 1.0)
    }
}
