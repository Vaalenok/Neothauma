use wgpu::*;
use wgpu::util::*;
use crate::engine::render::mesh::*;
use crate::engine::render::camera::*;
use crate::engine::render::transform::*;
use crate::engine::core::primitives::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    normal: Mat4,
    camera_pos: Vec3,
    shadow_mat: Mat4,
    _pad: u32
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            model: Mat4::default(),
            view: Mat4::default(),
            projection: Mat4::default(),
            normal: Mat4::default(),
            camera_pos: Vec3::ZERO,
            shadow_mat: Mat4::default(),
            _pad: 0
        }
    }
}

pub struct RenderableMesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Option<Buffer>,
    pub index_count: u32,
    pub uniform_buffer: Buffer,
    pub main_bind_group: BindGroup,
    pub shadow_bind_group: BindGroup
}

impl RenderableMesh {
    pub fn new(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        shadow_bind_group_layout: &BindGroupLayout,
        mesh: &Mesh,
        shadow_view: &TextureView,
        shadow_sampler: &Sampler,
        light_buffer: &Buffer,
        light_count_buffer: &Buffer
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: BufferUsages::VERTEX
        });

        let index_buffer = if !mesh.indices.is_empty() {
            Some(device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: BufferUsages::INDEX
            }))
        } else {
            None
        };

        let uniform_data = Uniforms::default();

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniform_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let main_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 1,
                    resource: light_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 2,
                    resource: light_count_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::TextureView(shadow_view),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::Sampler(shadow_sampler),
                }
            ],
            label: Some("uniform_bind_group")
        });

        let shadow_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: shadow_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding()
                }
            ],
            label: Some("shadow_bind_group")
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: mesh.index_count(),
            uniform_buffer,
            main_bind_group,
            shadow_bind_group
        }
    }

    pub fn update_uniforms(
        &self,
        queue: &Queue,
        transform: &Transform,
        camera: &Option<Camera>,
        aspect_ratio: f32,
        shadow_mat: Mat4
    ) {
        let camera = camera.clone().unwrap();

        let model = Mat4::from_transform(transform);
        let view = camera.get_view_matrix();
        let projection = camera.get_projection_matrix(aspect_ratio);

        let normal = model.inverse();

        let camera_pos = camera.position;

        let uniform_data = Uniforms {
            model,
            view,
            projection,
            normal,
            camera_pos,
            shadow_mat,
            ..Default::default()
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform_data));
    }

    pub fn update_uniforms_for_shadow(&self, queue: &Queue, transform: &Transform, light_view_proj_mat: Mat4) {
        let model = Mat4::from_transform(transform);

        let uniform_data = Uniforms {
            model,
            view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
            normal: model.inverse(),
            camera_pos: Vec3::ZERO,
            shadow_mat: light_view_proj_mat,
            _pad: 0
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform_data));
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: Vec3,
    pub _pad1: f32,
    pub color: Vec3,
    pub _pad2: f32,
    pub intensity: f32,
    pub _pad3: [f32; 7]
}

impl Light {
    pub fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            color: Vec3::IDENTITY,
            intensity: 1.0,
            _pad1: 0.0,
            _pad2: 0.0,
            _pad3: [0.0; 7]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightCount {
    pub count: u32
}
