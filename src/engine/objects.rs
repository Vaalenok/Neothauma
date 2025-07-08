use crate::engine::renderer::transform::*;
use crate::engine::renderer::mesh::*;
use crate::engine::ecs::*;
use crate::engine::engine::*;

// Куб
pub fn cube(engine: &mut Engine) -> Entity {
    let entity = engine.create_entity();

    let transform = Transform::default();
    engine.add_transform(entity, transform);

    let mesh = Mesh::cube();
    engine.add_mesh(entity, mesh);

    entity
}
