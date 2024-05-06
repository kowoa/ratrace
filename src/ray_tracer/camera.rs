use nalgebra_glm::{vec3, Vec3};

pub struct Camera {
    pub eye: Vec3,
    pub focal_length: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_center: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let eye = vec3(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = vec3(viewport_width, 0.0, 0.0);
        let viewport_v = vec3(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;
        let viewport_top_left =
            eye - viewport_u / 2.0 - viewport_v / 2.0 - vec3(0.0, 0.0, focal_length);
        // Center of the first pixel at the top left corner of the viewport
        let pixel00_center = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            eye,
            focal_length,
            viewport_width,
            viewport_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_center,
        }
    }
}
