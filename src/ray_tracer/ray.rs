use nalgebra_glm::{vec3, Vec3};

use super::color::Color;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Returns the color of the pixel that the ray hits
    pub fn trace(&self) -> Color {
        if self.hit_sphere(vec3(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    fn hit_sphere(&self, center: Vec3, radius: f32) -> bool {
        let oc = center - self.origin;
        let a = self.direction.magnitude_squared();
        let b = -2.0 * self.direction.dot(&oc);
        let c = oc.magnitude_squared() - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }
}
