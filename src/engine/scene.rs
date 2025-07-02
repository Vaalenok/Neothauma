use wgpu::util::DeviceExt;
use crate::engine::ecs::{Entity, Id};
use crate::engine::primitives::*;
use crate::engine::renderer::Camera;
use crate::engine::transform::*;
use crate::engine::uniforms::Uniforms;

pub struct RenderableMesh {
    pub id: Id,
    pub mesh: Mesh,
    pub transform: Transform,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: u32,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup
}

impl RenderableMesh {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        id: Id,
        entity: &Entity
    ) -> RenderableMesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&entity.mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = if !entity.mesh.indices.is_empty() {
            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&entity.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            }))
        } else {
            None
        };

        let uniform = Uniforms::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        RenderableMesh {
            id,
            mesh: entity.mesh.clone(),
            transform: entity.transform.clone(),
            vertex_buffer,
            index_buffer,
            index_count: entity.mesh.index_count(),
            uniform_buffer,
            bind_group,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, camera: &Camera, aspect_ratio: f32) {
        let model = Mat4::from_transform(
            self.transform.position,
            self.transform.rotation,
            self.transform.scale
        );

        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix(aspect_ratio);

        let mvp = proj * view * model;

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&mvp));
    }
}

pub struct Scene {
    pub objects: Vec<RenderableMesh>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: vec![]
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, pipeline: &wgpu::RenderPipeline) {
        pass.set_pipeline(pipeline);

        for obj in &self.objects {
            pass.set_bind_group(0, &obj.bind_group, &[]);
            pass.set_vertex_buffer(0, obj.vertex_buffer.slice(..));
            if let Some(ref index_buffer) = obj.index_buffer {
                pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..obj.index_count, 0, 0..1);
            } else {
                pass.draw(0..obj.index_count, 0..1);
            }
        }
    }
}
