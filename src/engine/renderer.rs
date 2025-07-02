use std::sync::Arc;
use wgpu::{RenderPassDescriptor, Surface, SurfaceConfiguration};
use winit::window::Window;
use crate::engine::{primitives::*, scene::*};

// Камера
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            0.1,
            100.0
        )
    }
}

impl Camera {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        up: Vec3,
        fov: f32,
        near: f32,
        far: f32
    ) -> Self {
        Self {
            position,
            direction,
            up,
            fov,
            near,
            far
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let f = self.direction.normalize();
        let s = f.cross(self.up).normalize();
        let u = s.cross(f);

        let mut result = Mat4::default();

        result.data[0][0] = s.x;
        result.data[1][0] = s.y;
        result.data[2][0] = s.z;
        result.data[3][0] = -s.dot(self.position);

        result.data[0][1] = u.x;
        result.data[1][1] = u.y;
        result.data[2][1] = u.z;
        result.data[3][1] = -u.dot(self.position);

        result.data[0][2] = -f.x;
        result.data[1][2] = -f.y;
        result.data[2][2] = -f.z;
        result.data[3][2] = f.dot(self.position);

        result.data[0][3] = 0.0;
        result.data[1][3] = 0.0;
        result.data[2][3] = 0.0;
        result.data[3][3] = 1.0;

        result
    }

    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        let fov_rad = self.fov.to_radians();
        let f = 1.0 / (fov_rad / 2.0).tan();

        let mut result = Mat4::default();
        result.data[0][0] = f / aspect_ratio;
        result.data[1][1] = f;
        result.data[2][2] = self.far / (self.near - self.far);
        result.data[3][2] = (self.near * self.far) / (self.near - self.far);
        result.data[2][3] = -1.0;
        result.data[3][3] = 0.0;

        result
    }

    pub fn move_forward(&mut self, dist: f32) {
        self.position = self.position + self.direction * dist;
    }

    pub fn move_backward(&mut self, dist: f32) {
        self.position = self.position - self.direction * dist;
    }

    pub fn move_right(&mut self, dist: f32) {
        let right = self.direction.cross(self.up).normalize();
        self.position = self.position + right * dist;
    }

    pub fn move_left(&mut self, dist: f32) {
        let right = self.direction.cross(self.up).normalize();
        self.position = self.position - right * dist;
    }

    pub fn rotate_yaw(&mut self, angle_rad: f32) {
        let rotation = Quat::from_axis_angle(self.up, angle_rad);
        self.direction = rotation * self.direction;
    }

    pub fn rotate_pitch(&mut self, angle_rad: f32) {
        let right = self.direction.cross(self.up).normalize();
        let rotation = Quat::from_axis_angle(right, angle_rad);
        self.direction = rotation * self.direction;
        self.direction = self.direction.normalize();
    }
}

// Рендерер
pub struct Renderer<'a> {
    window: Arc<Window>,
    surface: Surface<'a>,
    pub scene: Scene,
    camera: Camera,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline
}

impl<'a> Renderer<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("Не удалось создать поверхность");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Нет подходящего графического адаптера");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("Не удалось создать логическое устройство");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 0,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![]
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/default.wgsl").into()),
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: size_of::<[f32; 3]>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Option::from("vs_main"),
                compilation_options: Default::default(),
                buffers: &[vertex_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Option::from("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None
        });

        Self {
            window,
            surface,
            scene: Scene::new(),
            camera: Camera::default(),
            device,
            queue,
            config,
            size,
            render_pipeline
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
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        let aspect_ratio = self.size.width as f32 / self.size.height as f32;

        for obj in &self.scene.objects {
            obj.update_uniforms(&self.queue, &self.camera, aspect_ratio);
        }

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            });

            self.scene.draw(&mut render_pass, &self.render_pipeline);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
