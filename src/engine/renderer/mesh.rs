use crate::engine::core::primitives::*;

// Вершина
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }
}

// Полигональная сетка
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
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
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

    pub fn index_count(&self) -> u32 {
        self.indices.len() as u32
    }

    pub fn generate_normals(&mut self) {
        for vertex in &mut self.vertices {
            vertex.normal = Vec3::ZERO;
        }

        for triangle in self.indices.chunks(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;

            let v0 = self.vertices[i0].position;
            let v1 = self.vertices[i1].position;
            let v2 = self.vertices[i2].position;

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(edge2).normalize();

            self.vertices[i0].normal = self.vertices[i0].normal + normal;
            self.vertices[i1].normal = self.vertices[i1].normal + normal;
            self.vertices[i2].normal = self.vertices[i2].normal + normal;
        }

        for vertex in &mut self.vertices {
            vertex.normal = vertex.normal.normalize();
        }
    }

    pub fn cube() -> Mesh {
        let positions = vec![
            Vec3::new(-0.5, -0.5,  0.5), // 0
            Vec3::new( 0.5, -0.5,  0.5), // 1
            Vec3::new( 0.5,  0.5,  0.5), // 2
            Vec3::new(-0.5,  0.5,  0.5), // 3
            Vec3::new(-0.5, -0.5, -0.5), // 4
            Vec3::new( 0.5, -0.5, -0.5), // 5
            Vec3::new( 0.5,  0.5, -0.5), // 6
            Vec3::new(-0.5,  0.5, -0.5), // 7
        ];

        let vertices = positions
            .into_iter()
            .map(|pos| Vertex::new(pos, Vec3::ZERO))
            .collect::<Vec<_>>();

        let indices = vec![
            0, 1, 2,  2, 3, 0,
            5, 4, 7,  7, 6, 5,
            4, 0, 3,  3, 7, 4,
            1, 5, 6,  6, 2, 1,
            3, 2, 6,  6, 7, 3,
            4, 5, 1,  1, 0, 4
        ];

        let mut mesh = Mesh::new(vertices, indices);
        mesh.generate_normals();

        mesh
    }
}
