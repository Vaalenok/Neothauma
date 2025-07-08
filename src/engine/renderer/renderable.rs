use wgpu::*;
use wgpu::util::*;
use crate::engine::renderer::mesh::*;
use crate::engine::renderer::camera::*;
use crate::engine::renderer::transform::*;
use crate::engine::core::primitives::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    light_pos: Vec3,
    light_pad: f32,
    camera_pos: Vec3,
    camera_pad: f32
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            model: Mat4::default(),
            view: Mat4::default(),
            projection: Mat4::default(),
            light_pos: Vec3::ZERO,
            camera_pos: Vec3::ZERO,
            light_pad: 0.0,
            camera_pad: 0.0
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: Vec3,
    pub padding: f32,
    pub color: Vec3,
    pub intensity: f32
}

impl Light {
    pub(crate) fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            padding: 0.0,
            color: Vec3::IDENTITY,
            intensity: 1.0
        }
    }
}

pub struct RenderableMesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Option<Buffer>,
    pub index_count: u32,
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup
}

impl RenderableMesh {
    pub fn new(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        mesh: &Mesh,
        light_buffer: &Buffer
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

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 1,
                    resource: light_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group")
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: mesh.index_count(),
            uniform_buffer,
            bind_group
        }
    }

    pub fn update_uniforms(
        &self,
        queue: &Queue,
        transform: &Transform,
        camera: &Option<Camera>,
        aspect_ratio: f32
    ) {
        let camera = camera.clone().unwrap();
        
        let model = Mat4::from_transform(transform).to_uniform();
        let view = camera.get_view_matrix().to_uniform();
        let projection = camera.get_projection_matrix(aspect_ratio).to_uniform();
        let camera_pos = camera.position;

        let light_pos = Vec3::new(0.0, 0.0, 3.0);

        let uniform_data = Uniforms {
            model,
            view,
            projection,
            camera_pos,
            light_pos,
            ..Default::default()
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform_data));
    }
}
