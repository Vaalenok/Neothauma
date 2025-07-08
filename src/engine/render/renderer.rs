use std::sync::Arc;
use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;
use crate::engine::ecs::*;
use crate::engine::render::renderable::*;
use std::mem::size_of;

/// Максимальное число источников света. Обновлять вместе с шейдером
const MAX_LIGHTS: usize = 100;

pub struct Renderer<'a> {
    surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: RenderPipeline,
    depth_view: TextureView,
    pub light_buffer: Buffer,
    pub light_count_buffer: Buffer
}

impl<'a> Renderer<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("Не удалось создать поверхность");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Нет подходящего графического адаптера");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .expect("Не удалось создать логическое устройство");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 0,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![]
        };
        surface.configure(&device, &config);

        let depth_format = TextureFormat::Depth24PlusStencil8;

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: depth_format,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[depth_format]
        });

        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/default.wgsl").into())
        });

        let vertex_layout = VertexBufferLayout {
            array_stride: size_of::<[f32; 6]>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3
                },
                VertexAttribute {
                    offset: size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                    format: VertexFormat::Float32x3
                }
            ]
        };

        let uniform_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });

        let light_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Light Buffer"),
            size: (size_of::<Light>() * MAX_LIGHTS) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let light_count_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Light Count Buffer"),
            contents: bytemuck::bytes_of(&LightCount { count: 0 }),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Option::from("vs_main"),
                buffers: &[vertex_layout],
                compilation_options: Default::default()
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Option::from("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL
                })],
                compilation_options: Default::default()
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default()
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            depth_view,
            light_buffer,
            light_count_buffer
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

    pub fn render(&mut self, ecs: &mut ECS) -> Result<(), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        let aspect_ratio = self.size.width as f32 / self.size.height as f32;

        let lights = ecs.collect_lights();
        let light_count = LightCount { count: lights.len() as u32 };
        
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&lights));
        self.queue.write_buffer(&self.light_count_buffer, 0, bytemuck::bytes_of(&light_count));

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store
                    }
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                timestamp_writes: None,
                occlusion_query_set: None
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for (entity, renderable) in &ecs.renderables {
                if let Some(transform) = ecs.transforms.get(entity) {
                    renderable.update_uniforms(&self.queue, transform, &ecs.camera, aspect_ratio);
                    
                    render_pass.set_bind_group(0, &renderable.bind_group, &[]);
                    render_pass.set_vertex_buffer(0, renderable.vertex_buffer.slice(..));
                    
                    if let Some(index_buffer) = &renderable.index_buffer {
                        render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                        render_pass.draw_indexed(0..renderable.index_count, 0, 0..1);
                    } else {
                        render_pass.draw(0..renderable.index_count, 0..1);
                    }
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        
        // TODO: Убрать
        if let Some(camera) = &ecs.camera {
            println!("Pos: {:?} | Rot: {:?} | FOV: {}", camera.position, camera.rotation, camera.fov);
        }

        Ok(())
    }
}
