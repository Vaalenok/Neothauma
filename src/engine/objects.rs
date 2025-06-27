use crate::engine::primitives::*;

// Куб
pub fn create_cube() -> Mesh {
    
    let vertices = vec![
        Vec3::new(-0.5, -0.5, -0.5), // 0
        Vec3::new( 0.5, -0.5, -0.5), // 1
        Vec3::new( 0.5,  0.5, -0.5), // 2
        Vec3::new(-0.5,  0.5, -0.5), // 3
        Vec3::new(-0.5, -0.5,  0.5), // 4
        Vec3::new( 0.5, -0.5,  0.5), // 5
        Vec3::new( 0.5,  0.5,  0.5), // 6
        Vec3::new(-0.5,  0.5,  0.5), // 7
    ];

    let indices: Vec<u32> = vec![
        0, 1,  1, 2,  2, 3,  3, 0, // задняя грань
        4, 5,  5, 6,  6, 7,  7, 4, // передняя грань
        0, 4,  1, 5,  2, 6,  3, 7, // соединение переда и зада
    ];

    Mesh::new(vertices, indices)
}
