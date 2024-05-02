mod app;
mod renderer;

use app::App;
use color_eyre::eyre::Result;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn run() -> Result<()> {
    color_eyre::install()?;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug)?;
        } else {
            env_logger::init();
        }
    }

    let mut app = App::default();
    app.run()?;

    Ok(())
}

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn run_wasm() {
    run().unwrap();
}
