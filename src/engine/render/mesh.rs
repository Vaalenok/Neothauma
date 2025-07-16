use crate::engine::core::primitives::*;

/// Вершина
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

/// Полигональная сетка
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
            Vec3::new(-0.5,  0.5, -0.5)  // 7
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

    pub fn cone(segments: u16) -> Mesh {
        let radius = 0.5;
        let height = 1.0;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let tip = Vec3::new(0.0, height / 2.0, 0.0);
        vertices.push(Vertex::new(tip, Vec3::ZERO));

        let base_center = Vec3::new(0.0, -height / 2.0, 0.0);
        vertices.push(Vertex::new(base_center, Vec3::ZERO));

        for i in 0..segments {
            let theta = i as f32 / segments as f32 * std::f32::consts::TAU;
            let x = radius * theta.cos();
            let z = radius * theta.sin();
            let pos = Vec3::new(x, -height / 2.0, z);
            vertices.push(Vertex::new(pos, Vec3::ZERO));
        }

        for i in 0..segments {
            let next = if i + 1 < segments { i + 1 } else { 0 };
            indices.push(0);
            indices.push(2 + next);
            indices.push(2 + i);
        }

        for i in 0..segments {
            let next = if i + 1 < segments { i + 1 } else { 0 };
            indices.push(1);
            indices.push(2 + i);
            indices.push(2 + next);
        }

        let mut mesh = Mesh::new(vertices, indices);
        mesh.generate_normals();
        
        mesh
    }

    pub fn cylinder(segments: u16) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for &y in &[-0.5, 0.5] {
            for &r in &[1.0, 0.0] {
                for i in 0..segments {
                    let theta = i as f32 / segments as f32 * std::f32::consts::TAU;
                    let x = r * theta.cos();
                    let z = r * theta.sin();
                    vertices.push(Vertex::new(Vec3::new(x, y, z), Vec3::ZERO));
                }
            }
        }

        for i in 0..segments {
            let next = (i + 1) % segments;
            let outer_top = i;
            let inner_top = segments + i;
            let outer_top_next = next;
            let inner_top_next = segments + next;

            indices.push(outer_top);
            indices.push(inner_top_next);
            indices.push(inner_top);

            indices.push(outer_top);
            indices.push(outer_top_next);
            indices.push(inner_top_next);
        }

        let base = 2 * segments;
        for i in 0..segments {
            let next = (i + 1) % segments;
            let outer_bottom = base + i;
            let inner_bottom = base + segments + i;
            let outer_bottom_next = base + next;
            let inner_bottom_next = base + segments + next;

            indices.push(outer_bottom);
            indices.push(inner_bottom);
            indices.push(inner_bottom_next);

            indices.push(outer_bottom);
            indices.push(inner_bottom_next);
            indices.push(outer_bottom_next);
        }

        for i in 0..segments {
            let next = (i + 1) % segments;

            let outer_top = i;
            let outer_bottom = base + i;
            let outer_top_next = next;
            let outer_bottom_next = base + next;

            indices.push(outer_top);
            indices.push(outer_bottom);
            indices.push(outer_bottom_next);

            indices.push(outer_top);
            indices.push(outer_bottom_next);
            indices.push(outer_top_next);

            let inner_top = segments + i;
            let inner_bottom = base + segments + i;
            let inner_top_next = segments + next;
            let inner_bottom_next = base + segments + next;

            indices.push(inner_top);
            indices.push(inner_bottom_next);
            indices.push(inner_bottom);

            indices.push(inner_top);
            indices.push(inner_top_next);
            indices.push(inner_bottom_next);
        }

        let mut mesh = Mesh::new(vertices, indices);
        mesh.generate_normals();
        
        mesh
    }

    pub fn sphere(segments: u16) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for y in 0..=segments {
            let v = y as f32 / segments as f32;
            let theta = v * std::f32::consts::PI;

            for x in 0..=segments {
                let u = x as f32 / segments as f32;
                let phi = u * std::f32::consts::TAU;

                let sin_theta = theta.sin();
                let cos_theta = theta.cos();
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let pos = Vec3 {
                    x: sin_theta * cos_phi,
                    y: cos_theta,
                    z: sin_theta * sin_phi,
                };

                let normal = pos.normalize();
                
                vertices.push(Vertex::new(pos, normal));
            }
        }

        let ring = segments + 1;
        for y in 0..segments {
            for x in 0..segments {
                let i = y * ring + x;
                indices.push(i);
                indices.push(i + 1);
                indices.push(i + ring);

                indices.push(i + 1);
                indices.push(i + ring + 1);
                indices.push(i + ring);
            }
        }

        Mesh { vertices, indices }
    }
}
