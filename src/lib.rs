mod app;
mod renderer;

#[cfg(not(target_arch = "wasm32"))]
mod ray_tracer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        //pollster::block_on(run_async())
        ray_tracer::run();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run_async() {
    color_eyre::install().unwrap();

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).unwrap();
        } else {
            env_logger::init();
        }
    }

    let app = app::App::new().unwrap();
    app.run().await.unwrap();
}
