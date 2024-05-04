use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;

pub fn run() {
    color_eyre::install().unwrap();
    env_logger::init();

    let window = Window::new("Ray Tracer", 800, 600, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {}
}
