use color_eyre::eyre::Result;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Viewport<'window> {
    window: &'window Window,
    surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    background: wgpu::Color,
}

impl<'window> Viewport<'window> {
    pub fn new(
        window: &'window Window,
        background: wgpu::Color,
        surface: wgpu::Surface<'window>,
        adapter: &wgpu::Adapter,
    ) -> Result<Viewport<'window>> {
        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            window,
            surface,
            config,
            size,
            background,
        })
    }

    pub fn get_window(&self) -> &Window {
        self.window
    }

    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn get_background(&self) -> wgpu::Color {
        self.background
    }

    pub fn get_current_texture(
        &self,
    ) -> core::result::Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, device: &wgpu::Device) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(device, &self.config);
        }
    }
}
