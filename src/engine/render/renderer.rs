use std::sync::Arc;
use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;
use crate::engine::ecs::*;
use crate::engine::render::renderable::*;
use std::mem::size_of;
use wgpu::StoreOp::Store;
use crate::engine::core::primitives::*;

/// Максимальное число источников света. Обновлять вместе с шейдером
const MAX_LIGHTS: usize = 100;
/// Разрешение теней
const SHADOW_RESOLUTION: usize = 1024;

pub struct Renderer<'a> {
    surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: RenderPipeline,
    pub shadow_pipeline: RenderPipeline,
    pub depth_view: TextureView,
    pub shadow_cube_view: TextureView,
    pub shadow_cube_faces: Vec<TextureView>,
    pub shadow_sampler: Sampler,
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

        let depth_format = TextureFormat::Depth32Float;

        let shadow_cube_texture = device.create_texture(&TextureDescriptor {
            label: Some("Shadow Cube Texture"),
            size: Extent3d {
                width: SHADOW_RESOLUTION as u32,
                height: SHADOW_RESOLUTION as u32,
                depth_or_array_layers: 6
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        });

        let shadow_cube_view = shadow_cube_texture.create_view(&TextureViewDescriptor {
            label: Some("Shadow Cube View"),
            dimension: Some(TextureViewDimension::Cube),
            ..Default::default()
        });

        let shadow_cube_faces = (0..6).map(|i| {
            shadow_cube_texture.create_view(&TextureViewDescriptor {
                label: Some(&format!("Shadow Face {}", i)),
                dimension: Some(TextureViewDimension::D2),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            })
        }).collect::<Vec<_>>();

        let shadow_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Shadow Sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            compare: Some(CompareFunction::Less),
            ..Default::default()
        });

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Main Depth Texture"),
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[]
        });
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        let light_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Light Buffer"),
            size: (size_of::<Light>() * MAX_LIGHTS) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let light_count_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Light Count Buffer"),
            contents: bytemuck::bytes_of(&LightCount { count: 0 }),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::STORAGE
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

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Main Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/main.wgsl").into())
        });

        let shadow_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shadow Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/shadow.wgsl").into())
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[
                // 0 - Uniforms (model/view/proj)
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
                // 1 - Light array
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
                // 2 - Light count
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                // 3 - Shadow texture
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::Cube,
                        sample_type: TextureSampleType::Depth
                    },
                    count: None
                },
                // 4 - Shadow sampler
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Comparison),
                    count: None
                }
            ]
        });

        let shadow_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Shadow Bind Group Layout"),
            entries: &[
                // 0 - Uniforms
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[]
        });

        let shadow_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Shadow Pipeline Layout"),
            bind_group_layouts: &[&shadow_bind_group_layout], // Используем новый макет
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Option::from("vs_main"),
                buffers: &[vertex_layout.clone()],
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
            multiview: None,
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
            multisample: Default::default(),
            cache: None
        });

        let shadow_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Shadow Pipeline"),
            layout: Some(&shadow_pipeline_layout),
            vertex: VertexState {
                module: &shadow_shader,
                entry_point: Option::from("vs_main"),
                compilation_options: Default::default(),
                buffers: &[vertex_layout]
            },
            fragment: None,
            multiview: None,
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0
                }
            }),
            multisample: Default::default(),
            cache: None
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            shadow_pipeline,
            depth_view,
            shadow_cube_view,
            shadow_cube_faces,
            shadow_sampler,
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
        let aspect_ratio = self.size.width as f32 / self.size.height as f32;

        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let lights = ecs.collect_lights();
        let light_count = LightCount { count: lights.len() as u32 };

        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&lights));
        self.queue.write_buffer(&self.light_count_buffer, 0, bytemuck::bytes_of(&light_count));

        if lights.is_empty() {
            return Ok(());
        }

        let light_pos = Vec3::from(lights[0].position);
        let light_far_plane = 1000.0;

        let directions = [
            (Vec3::X, -Vec3::Y),
            (-Vec3::X, -Vec3::Y),
            (Vec3::Y, Vec3::Z),
            (-Vec3::Y, -Vec3::Z),
            (Vec3::Z, -Vec3::Y),
            (-Vec3::Z, -Vec3::Y)
        ];

        let proj = Mat4::perspective(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 1000.0);
        let light_matrices: Vec<Mat4> = directions.iter().map(|(dir, up)| {
            let view = Mat4::look_at(light_pos, light_pos + *dir, *up);
            proj * view
        }).collect();

        let mut shadow_encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Shadow Encoder")
        });

        for face in 0..6 {
            let mut shadow_pass = shadow_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some(&format!("Shadow Pass Face {}", face)),
                color_attachments: &[],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.shadow_cube_faces[face],
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: Store
                    }),
                    stencil_ops: None
                }),
                ..Default::default()
            });

            shadow_pass.set_pipeline(&self.shadow_pipeline);

            for (entity, renderable) in &ecs.renderables {
                if let Some(transform) = ecs.transforms.get(entity) {
                    renderable.update_uniforms_for_shadow(&self.queue, transform, light_matrices[face]);
                    shadow_pass.set_bind_group(0, &renderable.shadow_bind_group, &[]);
                    shadow_pass.set_vertex_buffer(0, renderable.vertex_buffer.slice(..));
                    if let Some(index_buffer) = &renderable.index_buffer {
                        shadow_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                        shadow_pass.draw_indexed(0..renderable.index_count, 0, 0..1);
                    } else {
                        shadow_pass.draw(0..renderable.index_count, 0..1);
                    }
                }
            }
        }

        self.queue.submit(Some(shadow_encoder.finish()));

        let mut main_encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Main Render Encoder")
        });

        {
            let mut render_pass = main_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: Store
                    }
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: Store
                    }),
                    stencil_ops: None
                }),
                ..Default::default()
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for (entity, renderable) in &ecs.renderables {
                if let Some(transform) = ecs.transforms.get(entity) {
                    renderable.update_uniforms(
                        &self.queue,
                        transform,
                        &ecs.camera.clone().unwrap(),
                        aspect_ratio,
                        light_pos,
                        light_far_plane,
                        light_matrices[0]
                    );

                    render_pass.set_bind_group(0, &renderable.main_bind_group, &[]);

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

        self.queue.submit(Some(main_encoder.finish()));
        frame.present();

        // TODO: убрать после отладки
        if let Some(camera) = &ecs.camera {
            println!("Pos: {:?} | Rot: {:?} | FOV: {}", camera.position, camera.rotation, camera.fov);
        }

        Ok(())
    }
}