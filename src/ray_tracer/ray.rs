use nalgebra_glm::{vec3, Vec3};

use super::{color::Color, hittable::Hittable, scene::Scene};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Returns the color of the pixel that the ray hits
    pub fn trace(&self, scene: &Scene) -> Color {
        for object in &scene.objects {
            if let Some(hit) = object.hit(self, f32::MIN, f32::MAX) {
                let normal = hit.normal;
                return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
            }
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
