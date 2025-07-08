use std::collections::HashSet;
use std::sync::*;
use winit::keyboard::PhysicalKey;
use winit::window::*;
use crate::engine::ecs::*;
use crate::engine::render::camera::Camera;
use crate::engine::render::mesh::*;
use crate::engine::render::renderable::Light;
use crate::engine::render::renderer::*;
use crate::engine::render::transform::*;

pub struct Engine<'a> {
    pub window: Arc<Window>,
    pub renderer: Renderer<'a>,
    pub ecs: ECS,
    pub pressed_keys: HashSet<PhysicalKey>
}

impl<'a> Engine<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        let ecs = ECS::new();
        let pressed_keys = HashSet::new();

        Self {
            window,
            renderer,
            ecs,
            pressed_keys
        }
    }
    
    // ECS
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        self.ecs.get_camera_mut()
    }
    
    pub fn create_entity(&mut self) -> Entity {
        self.ecs.create_entity()
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.ecs.delete_entity(entity);
    }

    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.ecs.add_transform(entity, transform);
    }

    pub fn add_mesh(&mut self, entity: Entity, mesh: Mesh) {
        self.ecs.add_mesh(entity, mesh, &self.renderer);
    }
    
    pub fn add_light(&mut self, entity: Entity, light: Light) {
        self.ecs.add_light(entity, light);
    }
    
    // Renderer
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&mut self.ecs)
    }
}
