use crate::engine::renderer::transform::*;
use crate::engine::renderer::mesh::*;
use crate::engine::ecs::*;
use crate::engine::engine::*;

// Куб
pub fn cube(engine: &mut Engine) -> Entity {
    let entity = engine.ecs.create_entity();

    let transform = Transform::default();
    engine.ecs.add_transform(entity, transform);

    let mesh = Mesh::cube();
    engine.ecs.add_mesh(entity, mesh, &engine.renderer);

    entity
}
