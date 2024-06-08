mod camera;
mod color;
mod hittable;
mod ray;
mod scene;

use color_eyre::eyre::{OptionExt, Result};
use image::{buffer::ConvertBuffer, ImageBuffer, Rgba, RgbaImage};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::vec3;
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use self::{camera::Camera, hittable::Sphere, ray::Ray, scene::Scene};

pub fn run_and_display() {
    color_eyre::install().unwrap();
    env_logger::init();

    let image_rgba = run().expect("Failed to create raytraced image");
    let image_width = image_rgba.width();
    let image_height = image_rgba.height();
    let image_rgba_raw = image_rgba.as_raw();

    // Reinterpret as BGRA image
    let image_bgra_raw: Vec<u8> = {
        let mut image_bgra_raw = Vec::with_capacity(image_rgba_raw.len());
        for chunk in image_rgba_raw.chunks_exact(4) {
            image_bgra_raw.push(chunk[2]);
            image_bgra_raw.push(chunk[1]);
            image_bgra_raw.push(chunk[0]);
            image_bgra_raw.push(chunk[3]);
        }
        image_bgra_raw
    };

    let mut window = Window::new(
        "Ray Tracer",
        image_width as usize,
        image_height as usize,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                bytemuck::cast_slice(&image_bgra_raw),
                image_width as usize,
                image_height as usize,
            )
            .unwrap();
    }
}

/// Returns a pixel buffer alongside width and height
pub fn run() -> Result<RgbaImage> {
    let image_aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = std::cmp::max((image_width as f32 / image_aspect_ratio) as u32, 1);

    let camera = Camera::new(image_width, image_height);
    let scene = {
        let mut scene = Scene::default();
        scene.add_object(Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }));
        scene.add_object(Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }));
        scene
    };

    let pixels: Vec<Rgba<u8>> = (0..image_width * image_height)
        .into_par_iter()
        .map_init(thread_rng, |_rng, i| {
            let u = i as f32 % image_width as f32;
            let v = i as f32 / image_width as f32;

            let pixel_center =
                camera.pixel00_center + u * camera.pixel_delta_u + v * camera.pixel_delta_v;
            let ray_direction = pixel_center - camera.eye;
            let ray = Ray::new(camera.eye, ray_direction);

            let pixel_color = ray.trace(&scene);
            pixel_color.as_rgba()
        })
        .collect();

    // Convert Vec<Rgba<u8>> into Vec<u8>
    let mut pixels_raw: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
    for pixel in pixels {
        pixels_raw.extend_from_slice(&pixel.0);
    }

    let image: RgbaImage = ImageBuffer::from_vec(image_width, image_height, pixels_raw)
        .ok_or_eyre("Failed to create image buffer")?;

    Ok(image)
}
