use crate::engine::render::transform::*;
use crate::engine::render::mesh::*;
use crate::engine::render::renderable::*;
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

// Освещение
pub fn light(engine: &mut Engine) -> Entity {
    let entity = engine.create_entity();

    let transform = Transform::default();
    engine.add_transform(entity, transform);
    
    let light = Light::default();
    engine.add_light(entity, light);
    
    entity
}
