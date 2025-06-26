use winit::window::Window;
use crate::engine::objects::primitives::{Drawable, Point, Vec3};

// Камера
pub struct Camera {
    pub position: Point,
    pub direction: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
            0.1,
            100.0
        )
    }
}

impl Camera {
    pub fn new(
        position: Point,
        direction: Vec3,
        up: Vec3,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32
    ) -> Self {
        Self {
            position,
            direction,
            up,
            fov,
            aspect_ratio,
            near,
            far
        }
    }

    // pub(crate) fn is_in_frustum(&self, obj: ) -> bool {
    //     todo!()
    // }
}

// Рендерер
pub struct Renderer<'a> {
    camera: Camera,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window)
            .expect("Не удалось создать поверхность");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Нет подходящего адаптера");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("Не удалось создать устройство");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

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

        surface.configure(&device, &config);

        Self {
            camera: Camera::default(),
            surface,
            device,
            queue,
            config,
            size
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });

        {
            let _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            });
        }

        // for obj in &self.scene_objects {
        //     if self.camera.is_in_frustum(obj.bounding_volume()) {
        //         obj.draw(&mut encoder, &view);
        //     }
        // }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
