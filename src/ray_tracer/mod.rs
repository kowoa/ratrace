mod camera;
mod color;
mod hittable;
mod ray;
mod scene;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::vec3;
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use self::{camera::Camera, ray::Ray, scene::Scene};

pub fn run() {
    color_eyre::install().unwrap();
    env_logger::init();

    let image_aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = std::cmp::max((image_width as f32 / image_aspect_ratio) as u32, 1);

    let camera = Camera::new(image_width, image_height);
    let scene = {
        let mut scene = Scene::default();
        scene.add_object(Box::new(hittable::Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }));
        scene
    };

    let buffer: Vec<u32> = (0..image_width * image_height)
        .into_par_iter()
        .map_init(thread_rng, |_rng, i| {
            let u = i as f32 % image_width as f32;
            let v = i as f32 / image_width as f32;

            let pixel_center =
                camera.pixel00_center + u * camera.pixel_delta_u + v * camera.pixel_delta_v;
            let ray_direction = pixel_center - camera.eye;
            let ray = Ray::new(camera.eye, ray_direction);

            let pixel_color = ray.trace(&scene);
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
