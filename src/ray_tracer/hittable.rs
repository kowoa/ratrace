use nalgebra_glm::Vec3;

use super::ray::Ray;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.magnitude_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= t_min || root >= t_max {
            root = (h + sqrtd) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let (normal, front_face) = {
            let outward_normal = (point - self.center) / self.radius;
            let front_face = ray.direction.dot(&outward_normal) < 0.0;
            (
                if front_face {
                    // Outside the sphere
                    outward_normal
                } else {
                    // Inside the sphere
                    -outward_normal
                },
                front_face,
            )
        };
        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
        })
    }
}
