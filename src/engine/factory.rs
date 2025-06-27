use wgpu::util::DeviceExt;
use crate::engine::ecs::{Entity, Id};
use crate::engine::scene::*;
use crate::engine::transform::*;
use crate::engine::primitives::*;

pub fn create_renderable_mesh(
    device: &wgpu::Device,
    bind_layout: &wgpu::BindGroupLayout,
    id: Id,
    entity: &Entity
) -> RenderableMesh {
    let mesh = Mesh {
        vertices: entity.mesh.vertices.clone(),
        indices: entity.mesh.indices.iter().map(|&idx| idx.to_be()).collect(),
    };

    let vertex_data: Vec<Vec3> = entity.mesh.vertices
        .iter()
        .map(|v| Vec3::new(v.x, v.y, v.z))
        .collect();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertex_data),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let indices = &entity.mesh.indices;

    let (index_buffer, index_count) = if !indices.is_empty() {
        let indices_u16: Vec<u16> = indices.iter().map(|&i| i as u16).collect();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices_u16),
            usage: wgpu::BufferUsages::INDEX,
        });

        (Some(buffer), indices_u16.len() as u32)
    } else {
        (None, entity.mesh.vertices.len() as u32)
    };

    let uniform = Mat4::IDENTITY;

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::bytes_of(&uniform),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bind_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
        label: Some("Bind Group"),
    });

    RenderableMesh {
        id,
        mesh,
        transform: Transform::default(),
        vertex_buffer,
        index_buffer,
        index_count,
        uniform_buffer,
        bind_group,
    }
}
