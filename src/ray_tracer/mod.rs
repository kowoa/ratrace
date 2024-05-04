mod ray;

use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn run() {
    color_eyre::install().unwrap();
    env_logger::init();

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    let buffer: Vec<u32> = (0..WIDTH * HEIGHT)
        .into_par_iter()
        .map_init(thread_rng, |rng, i| {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let r = x as f64 / (WIDTH - 1) as f64;
            let g = y as f64 / (HEIGHT - 1) as f64;
            let b = 0.0;

            color_f(r, g, b)
        })
        .collect();

    let mut window = Window::new(
        "Ray Tracer",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
}

fn color(r: u8, g: u8, b: u8) -> u32 {
    u32::from_le_bytes([b, g, r, 255])
}

fn color_f(r: f64, g: f64, b: f64) -> u32 {
    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;
    color(ir, ig, ib)
}
