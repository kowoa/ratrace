mod app;
mod renderer;

use color_eyre::eyre::Result;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub async fn run() -> Result<()> {
    color_eyre::install()?;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug)?;
        } else {
            env_logger::init();
        }
    }

    let app = app::App::new()?;
    app.run().await?;

    Ok(())
}

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run_wasm() {
    //wasm_bindgen_futures::spawn_local(run());
    run().await.unwrap();
}
