mod color;
mod ray;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{vec3, Vec3};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use self::ray::Ray;

struct Camera {
    eye: Vec3,
    focal_length: f32,
    viewport_width: f32,
    viewport_height: f32,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        Self {
            eye: Vec3::new(0.0, 0.0, 0.0),
            focal_length: 1.0,
            viewport_width,
            viewport_height,
        }
    }
}

pub fn run() {
    color_eyre::install().unwrap();
    env_logger::init();

    let image_aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = std::cmp::max((image_width as f32 / image_aspect_ratio) as u32, 1);

    let camera = Camera::new(image_width, image_height);
    let viewport_u = vec3(camera.viewport_width, 0.0, 0.0);
    let viewport_v = vec3(0.0, -camera.viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;
    let viewport_top_left =
        camera.eye - viewport_u / 2.0 - viewport_v / 2.0 - vec3(0.0, 0.0, camera.focal_length);
    // Center of the first pixel at the top left corner of the viewport
    let pixel00_center = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let buffer: Vec<u32> = (0..image_width * image_height)
        .into_par_iter()
        .map_init(thread_rng, |_rng, i| {
            let u = i as f32 % image_width as f32;
            let v = i as f32 / image_width as f32;

            let pixel_center = pixel00_center + u * pixel_delta_u + v * pixel_delta_v;
            let ray_direction = pixel_center - camera.eye;
            let ray = Ray::new(camera.eye, ray_direction);

            let pixel_color = ray.trace();
            pixel_color.as_u32()
        })
        .collect();

    let mut window = Window::new(
        "Ray Tracer",
        image_width as usize,
        image_height as usize,
        WindowOptions::default(),
    )
    .unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, image_width as usize, image_height as usize)
            .unwrap();
    }
}
