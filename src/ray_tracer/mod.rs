use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

pub fn run() {
    color_eyre::install().unwrap();
    env_logger::init();

    let buffer: Vec<u32> = (0..WIDTH * HEIGHT)
        //.into_par_iter()
        //.map_init(thread_rng, |rng, i| {
        .into_iter()
        .map(|i| {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let r = x as f64 / (WIDTH - 1) as f64;
            let g = y as f64 / (HEIGHT - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            // BGRA
            u32::from_le_bytes([ib, ig, ir, 255])
        })
        .collect();
    /*
        let mut buffer: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 4);
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let r = i as f64 / (WIDTH - 1) as f64;
                let g = j as f64 / (HEIGHT - 1) as f64;
                let b = 0.0;
                let ir = (255.999 * r) as u8;
                let ig = (255.999 * g) as u8;
                let ib = (255.999 * b) as u8;
                // BGRA
                buffer.push(ib);
                buffer.push(ig);
                buffer.push(ir);
                buffer.push(255);
            }
        }
    */

    let mut window = Window::new("Ray Tracer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
