use bytemuck::{Pod, Zeroable};
use crate::engine::primitives::Mat4;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub mvp: [[f32; 4]; 4]
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            mvp: Mat4::IDENTITY.to_cols_array_2d()
        }
    }

    pub fn update(&mut self, model: Mat4, view: Mat4, proj: Mat4) {
        let mvp = proj * view * model;
        self.mvp = mvp.to_cols_array_2d();
    }
}
