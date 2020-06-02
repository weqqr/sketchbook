use crate::shape::*;

pub struct Scene {
    shapes: Vec<Box<dyn Shape>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    pub fn add<S: Shape + 'static>(&mut self, s: S) {
        self.shapes.push(Box::new(s));
    }

    pub fn shapes(&self) -> impl Iterator<Item = &dyn Shape> {
        self.shapes.iter().map(|o| o.as_ref())
    }
}
