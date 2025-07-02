use crate::engine::factory::create_renderable_mesh;
use crate::engine::transform::*;
use crate::engine::primitives::*;
use crate::engine::renderer::Renderer;

pub type Id = usize;

#[derive(Clone)]
pub struct Entity {
    pub id: Id,
    pub name: String,
    pub transform: Transform,
    pub mesh: Mesh
}

impl Entity {
    pub fn new(id: Id, name: String) -> Self {
        Self { id, name, transform: Transform::default(), mesh: Mesh::default() }
    }
}

pub struct ECS {
    next_id: Id,
    entities: Vec<Entity>
}

impl ECS {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: Vec::new()
        }
    }
    
    pub fn create_entity(
        &mut self,
        name: String,
        mesh: Mesh,
        transform: Transform,
        renderer: &mut Renderer
    ) -> Entity {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity {
            id,
            name,
            transform,
            mesh: mesh.clone()
        };

        self.entities.push(entity.clone());

        let entity_ref = self.entities.last().unwrap();
        let renderable = create_renderable_mesh(
            &renderer.device,
            &renderer.render_pipeline.get_bind_group_layout(0),
            entity.id,
            entity_ref
        );
        renderer.scene.objects.push(renderable);

        entity
    }
}
