use crate::engine::objects::primitives::*;

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0)
        }
    }
}
