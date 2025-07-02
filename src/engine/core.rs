use std::sync::Arc;
use winit::window::Window;
use crate::engine::{ecs::ECS, renderer::Renderer, scene::Scene};

pub struct Engine<'a> {
    pub window: Arc<Window>,
    pub renderer: Renderer<'a>,
    pub ecs: ECS,
    pub scenes: Vec<Scene>,
    pub current_scene_index: usize
}

impl<'a> Engine<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        let ecs = ECS::new();
        let scenes = vec![Scene::new()];
        let current_scene_index = 0;

        Self {
            window,
            renderer,
            ecs,
            scenes,
            current_scene_index
        }
    }

    pub fn current_scene(&self) -> &Scene {
        &self.scenes[self.current_scene_index]
    }

    pub fn current_scene_mut(&mut self) -> &mut Scene {
        &mut self.scenes[self.current_scene_index]
    }

    pub fn switch_scene(&mut self, index: usize) {
        if index < self.scenes.len() {
            self.current_scene_index = index;
        }
    }
}
