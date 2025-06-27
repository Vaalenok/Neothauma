use crate::engine::ecs::*;
use crate::engine::primitives::Mesh;
use crate::engine::renderer::*;
use crate::engine::transform::Transform;

pub struct Engine<'a> {
    pub renderer: Renderer<'a>,
    pub ecs: ECS,
}

impl Engine<'_> {
    pub fn update_entity(&mut self, entity: &mut Entity) {
        self.ecs.update_entity(entity.clone(), &mut self.renderer)
    }
    
    pub fn create_entity(
        &mut self,
        name: impl Into<String>,
        mesh: Mesh,
        transform: Transform
    ) -> Entity {
        self.ecs.create_entity(name.into(), mesh, transform, &mut self.renderer)
    }
}
