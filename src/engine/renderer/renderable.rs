use wgpu::*;
use wgpu::util::*;
use crate::engine::renderer::mesh::*;
use crate::engine::renderer::camera::*;
use crate::engine::renderer::transform::*;
use crate::engine::core::primitives::*;

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
        mesh: &Mesh
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

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&Mat4::default()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding()
            }],
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
        camera: &Camera,
        aspect_ratio: f32
    ) {
        let model = Mat4::from_transform(transform);
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix(aspect_ratio);

        let mvp = proj * view * model;

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&mvp.to_uniform()));
    }
}
