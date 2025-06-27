use std::sync::Arc;
use crate::engine::objects::primitives::*;
use crate::engine::objects::shapes::*;
use crate::engine::transform::*;

pub struct RenderableMesh {
    pub meshes: Arc<Mesh>,
    pub transform: Transform,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: u32,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup
}

impl RenderableMesh {
    pub fn update_uniforms(&self, queue: &wgpu::Queue) {
        let uniform = Mat4::from_transform(
            self.transform.position,
            self.transform.rotation,
            self.transform.scale
        );
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform));
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
