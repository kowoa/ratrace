use color_eyre::eyre::Result;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowBuilder},
};

use crate::renderer::Renderer;

pub async fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let window_size = winit::dpi::LogicalSize::new(800.0, 600.0);
    let window = WindowBuilder::new()
        .with_title("Press R to toggle redraw requests.")
        .with_inner_size(window_size)
        .with_resizable(true)
        .build(&event_loop)?;

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(window_size);

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("canvas-container")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Failed to append canvas to element with id=\"canvas-container\"");
    }

    {
        let renderer = Renderer::new(&window).await?;

        let mut request_redraws = true;
        let mut close_requested = false;

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    window_id,
                    ref event,
                } if window_id == renderer.get_window().id() => match event {
                    WindowEvent::CloseRequested => close_requested = true,
                    WindowEvent::RedrawRequested => {
                        renderer.get_window().pre_present_notify();
                        // draw here
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: key,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match key.as_ref() {
                        Key::Character("r") => {
                            request_redraws = !request_redraws;
                            log::info!("request_redraws: {}", request_redraws);
                        }
                        Key::Named(NamedKey::Escape) => {
                            close_requested = true;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Event::AboutToWait => {
                    if request_redraws {
                        renderer.get_window().request_redraw();
                    }

                    if close_requested {
                        elwt.exit();
                    }
                }
                _ => {}
            };
        })?;
    }

    Ok(())
}
