use crate::image::Rgba;
use crate::math::*;
use std::ops::{Add, Sub, Mul, Div};

const GAMMA: Float = 2.2;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Color { r, g, b }
    }

    pub fn clamp(&self, min: Color, max: Color) -> Color {
        Color {
            r: clamp(self.r, min.r, max.r),
            g: clamp(self.g, min.g, max.g),
            b: clamp(self.b, min.b, max.b),
        }
    }
}

fn srgb_to_linear(component: Float) -> Float {
    component.powf(2.2)
}

fn linear_to_srgb(component: Float) -> Float {
    component.powf(1.0 / 2.2)
}

fn aces(x: Color) -> Color {
    let a = 2.51;
    let b = Color::new(0.03, 0.03, 0.03);
    let c = 2.43;
    let d = Color::new(0.59, 0.59, 0.59);
    let e = Color::new(0.14, 0.14, 0.14);
    ((x*(x*a+b))/(x*(x*c+d)+e)).clamp(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0))
}

impl From<Vector3> for Color {
    fn from(n: Vector3) -> Color {
        Color {
            r: n.x * 0.5 + 0.5,
            g: n.y * 0.5 + 0.5,
            b: n.z * 0.5 + 0.5,
        }
    }
}

impl From<Rgba> for Color {
    fn from(c: Rgba) -> Color {
        Color {
            r: srgb_to_linear(c.r as Float / 255.0),
            g: srgb_to_linear(c.g as Float / 255.0),
            b: srgb_to_linear(c.b as Float / 255.0),
        }
    }
}

impl Into<Rgba> for Color {
    fn into(self) -> Rgba {
        let c = aces(self);
        Rgba {
            r: (linear_to_srgb(c.r) * 255.0) as u8,
            g: (linear_to_srgb(c.g) * 255.0) as u8,
            b: (linear_to_srgb(c.b) * 255.0) as u8,
            a: 255,
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Div for Color {
    type Output = Self;
    fn div(self, rhs: Color) -> Color {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Mul<Float> for Color {
    type Output = Self;
    fn mul(self, rhs: Float) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<Float> for Color {
    type Output = Self;
    fn div(self, rhs: Float) -> Color {
        let reciprocal = 1.0 / rhs;
        Color {
            r: self.r * reciprocal,
            g: self.g * reciprocal,
            b: self.b * reciprocal,
        }
    }
}
