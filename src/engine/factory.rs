use std::sync::Arc;
use wgpu::util::DeviceExt;
use crate::engine::scene::*;
use crate::engine::transform::*;
use crate::engine::objects::primitives::*;
use crate::engine::objects::shapes::*;

pub fn create_renderable_mesh(
    device: &wgpu::Device,
    bind_layout: &wgpu::BindGroupLayout,
    vertices: &[Vec3],
    indices: Option<&[u16]>
) -> RenderableMesh {
    let triangles = if let Some(indices) = indices {
        indices.chunks(3)
            .map(|chunk| Triangle {
                v0: Vec3::new(
                    vertices[chunk[0] as usize].x,
                    vertices[chunk[0] as usize].y,
                    vertices[chunk[0] as usize].z
                ),
                v1: Vec3::new(
                    vertices[chunk[1] as usize].x,
                    vertices[chunk[1] as usize].y,
                    vertices[chunk[1] as usize].z
                ),
                v2: Vec3::new(
                    vertices[chunk[2] as usize].x,
                    vertices[chunk[2] as usize].y,
                    vertices[chunk[2] as usize].z
                )
            })
            .collect()
    } else {
        vec![]
    };

    let mesh = Arc::new(Mesh { triangles });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX
    });

    let (index_buffer, index_count) = if let Some(indices) = indices {
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX
        });
        (Some(ib), indices.len() as u32)
    } else {
        (None, vertices.len() as u32)
    };

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::bytes_of(&Mat4::IDENTITY),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bind_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding()
        }],
        label: Some("Bind Group")
    });

    RenderableMesh {
        meshes: mesh,
        transform: Transform::default(),
        vertex_buffer,
        index_buffer,
        index_count,
        uniform_buffer,
        bind_group
    }
}
