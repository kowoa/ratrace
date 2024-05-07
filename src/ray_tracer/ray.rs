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
        let sphere_center = vec3(0.0, 0.0, -1.0);
        let sphere_radius = 0.5;
        let t = self.hit_sphere(sphere_center, sphere_radius);
        if t > 0.0 {
            let normal = (self.at(t) - sphere_center).normalize();
            return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
        }

        // Sky gradient
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    /// Returns the value of t where the ray hits the sphere
    fn hit_sphere(&self, center: Vec3, radius: f32) -> f32 {
        let oc = center - self.origin;
        let a = self.direction.magnitude_squared();
        let h = self.direction.dot(&oc);
        let c = oc.magnitude_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (h - discriminant.sqrt()) / a
        }
    }
}
