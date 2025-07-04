use crate::engine::core::primitives::*;

// Полигональная сетка
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u16>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

    pub fn index_count(&self) -> u32 {
        self.indices.len() as u32
    }

    pub fn cube() -> Mesh {
        let vertices = vec![
            Vec3::new(-0.5, -0.5,  0.5), // 0
            Vec3::new( 0.5, -0.5,  0.5), // 1
            Vec3::new( 0.5,  0.5,  0.5), // 2
            Vec3::new(-0.5,  0.5,  0.5), // 3
            Vec3::new(-0.5, -0.5, -0.5), // 4
            Vec3::new( 0.5, -0.5, -0.5), // 5
            Vec3::new( 0.5,  0.5, -0.5), // 6
            Vec3::new(-0.5,  0.5, -0.5)  // 7
        ];

        let indices = vec![
            0, 1, 2,  2, 3, 0,
            5, 4, 7,  7, 6, 5,
            4, 0, 3,  3, 7, 4,
            1, 5, 6,  6, 2, 1,
            3, 2, 6,  6, 7, 3,
            4, 5, 1,  1, 0, 4
        ];

        Mesh::new(vertices, indices)
    }
}
