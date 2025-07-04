use std::sync::*;
use winit::window::*;
use crate::engine::ecs::*;
use crate::engine::renderer::renderer::*;

pub struct Engine<'a> {
    pub window: Arc<Window>,
    pub renderer: Renderer<'a>,
    pub ecs: ECS
}

impl<'a> Engine<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        let ecs = ECS::new();

        Self {
            window,
            renderer,
            ecs
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.ecs.update(dt)
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(&mut self.ecs)
    }
}
