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
        // Get the HitRecord of the closest object that the ray hits
        let t_min = 0.0;
        let t_max = f32::MAX;
        let mut hit_record = None;
        let mut t_closest = t_max;
        for object in &scene.objects {
            let hit = object.hit(self, t_min, t_closest);
            if hit.is_some() {
                hit_record = hit;
                t_closest = hit_record.as_ref().unwrap().t;
            }
        }

        // If the ray hit any object, return the color of the object
        if let Some(hit_record) = hit_record {
            return Color::from_vec3((hit_record.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5);
        }

        // Sky gradient
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
