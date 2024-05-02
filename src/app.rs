use color_eyre::eyre::Result;
use winit::window::WindowBuilder;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::Window,
};

use crate::renderer::Renderer;

pub async fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut window = Some(
        WindowBuilder::new()
            .with_title("Press R to toggle redraw requests.")
            .build(&event_loop)?,
    );

    event_loop.set_control_flow(ControlFlow::Poll);

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));

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

    let mut renderer = Some(Renderer::new(window.as_ref().unwrap()).await?);

    let mut request_redraws = true;
    let mut close_requested = false;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.as_ref().unwrap().id() => match event {
                WindowEvent::CloseRequested => close_requested = true,
                WindowEvent::RedrawRequested => {
                    window.as_ref().unwrap().pre_present_notify();
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
                    window.as_ref().unwrap().request_redraw();
                }

                if close_requested {
                    renderer = None;
                    window = None;
                    elwt.exit();
                }
            }
            _ => {}
        };
    });

    Ok(())
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attribs = Window::default_attributes().with_title("Press R to toggle redraw requests.");
        let window = event_loop.create_window(attribs).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));

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

        self.window = Some(window);

        pollster::block_on(self.renderer.init(self.window.as_ref().unwrap())).unwrap();
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::RedrawRequested => {
                window.pre_present_notify();
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
                    self.request_redraws = !self.request_redraws;
                    log::info!("request_redraws: {}", self.request_redraws);
                }
                Key::Named(NamedKey::Escape) => {
                    self.close_requested = true;
                }
                _ => (),
            },
            _ => {}
        };
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.request_redraws {
            self.window.as_ref().unwrap().request_redraw();
        }

        if self.close_requested {
            self.renderer = None;
            self.window = None;
            event_loop.exit();
        }
    }
}
