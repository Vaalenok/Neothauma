use std::collections::HashMap;
use crate::engine::renderer::mesh::*;
use crate::engine::renderer::transform::*;
use crate::engine::renderer::renderer::*;
use crate::engine::renderer::renderable::*;

pub trait Updatable {
    fn update(&mut self, dt: f32, transform: &mut Transform);
}

pub type Entity = usize;

pub struct ECS {
    next_entity: Entity,
    pub transforms: HashMap<Entity, Transform>,
    pub meshes: HashMap<Entity, Mesh>,
    pub scripts: HashMap<Entity, Box<dyn Updatable>>,
    pub renderables: HashMap<Entity, RenderableMesh>
}

impl ECS {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            transforms: HashMap::new(),
            meshes: HashMap::new(),
            scripts: HashMap::new(),
            renderables: HashMap::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_entity;
        self.next_entity += 1;
        id
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.transforms.remove(&entity);
        self.meshes.remove(&entity);
        self.scripts.remove(&entity);
        self.renderables.remove(&entity);
    }

    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transforms.insert(entity, transform);
    }

    pub fn add_mesh(&mut self, entity: Entity, mesh: Mesh, renderer: &Renderer) {
        self.meshes.insert(entity, mesh.clone());

        let renderable = RenderableMesh::new(
            &renderer.device,
            &renderer.render_pipeline.get_bind_group_layout(0),
            &mesh
        );
        
        self.renderables.insert(entity, renderable);
    }

    pub fn add_script(&mut self, entity: Entity, script: Box<dyn Updatable>) {
        self.scripts.insert(entity, script);
    }

    pub fn update(&mut self, dt: f32) {
        for (entity, script) in &mut self.scripts {
            if let Some(transform) = self.transforms.get_mut(entity) {
                script.update(dt, transform);
            }
        }
    }
}
