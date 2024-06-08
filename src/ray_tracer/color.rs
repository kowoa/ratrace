use std::ops::{Add, Mul};

use image::Rgba;
use nalgebra_glm::Vec3;

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }

    pub fn as_rgba(&self) -> Rgba<u8> {
        let r = (255.999 * self.r) as u8;
        let g = (255.999 * self.g) as u8;
        let b = (255.999 * self.b) as u8;
        [r, g, b, 255].into()
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
