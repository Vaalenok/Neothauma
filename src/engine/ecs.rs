use std::collections::HashMap;
use crate::engine::core::primitives::*;
use crate::engine::render::camera::*;
use crate::engine::render::mesh::*;
use crate::engine::render::renderable::*;
use crate::engine::render::renderer::*;
use crate::engine::render::transform::*;

pub type Entity = usize;

pub struct ECS {
    next_entity: Entity,
    pub camera: Option<Camera>,
    pub transforms: HashMap<Entity, Transform>,
    pub meshes: HashMap<Entity, Mesh>,
    pub renderables: HashMap<Entity, RenderableMesh>,
    pub lights: HashMap<Entity, Light>
}

impl ECS {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            camera: Some(Camera::default()),
            transforms: HashMap::new(),
            meshes: HashMap::new(),
            renderables: HashMap::new(),
            lights: HashMap::new()
        }
    }

    /// Camera
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        self.camera.as_mut().expect("Камеры нет")
    }

    /// Light
    pub fn collect_lights(&self) -> Vec<Light> {
        self.lights.values().cloned().collect()
    }

    pub fn add_light(&mut self, entity: Entity, light: Light) {
        self.lights.insert(entity, light);
    }
    
    pub fn edit_light(&mut self, entity: &Entity, color: Vec3, intensity: f32, range: f32) {
        if let Some(obj_light) = self.lights.get_mut(&entity) {
            obj_light.color = color;
            obj_light.intensity = intensity;
            obj_light.range = range;
        }
    }

    /// Entity
    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_entity;
        self.next_entity += 1;
        id
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.transforms.remove(&entity);
        self.meshes.remove(&entity);
    }

    /// Transform
    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transforms.insert(entity, transform);
    }

    pub fn transform(&mut self, entity: &Entity, transform: Transform) {
        if let Some(obj_transform) = self.transforms.get_mut(entity) {
            *obj_transform = transform;
        }
    }

    /// Mesh
    pub fn add_mesh(&mut self, entity: Entity, mesh: Mesh, renderer: &Renderer) {
        self.meshes.insert(entity, mesh.clone());

        let renderable = RenderableMesh::new(
            &renderer.device,
            &renderer.render_pipeline.get_bind_group_layout(0),
            &renderer.shadow_pipeline.get_bind_group_layout(0),
            &mesh,
            &renderer.shadow_cube_view, 
            &renderer.shadow_sampler,
            &renderer.light_buffer,
            &renderer.light_count_buffer
        );

        self.renderables.insert(entity, renderable);
    }
    
    // TODO: добавить скрипты
    // pub fn add_script(&mut self, entity: Entity, script: Box<dyn Updatable>) {
    //     self.scripts.insert(entity, script);
    // }
    // 
    // pub fn update(&mut self, dt: f32, engine: &mut Engine) {
    //     for script in &mut self.scripts {
    //         script.1.update(dt, engine);
    //     }
    // }
}
