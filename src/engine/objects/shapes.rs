use crate::engine::objects::primitives::*;

// Полигон
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3
}

// Полигональная сетка
pub struct Mesh {
    pub triangles: Vec<Triangle>
}
