use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop},
    window::{Window, WindowId},
};
use crate::engine::ecs::ECS;
use crate::engine::renderer::Renderer;
use crate::engine::core::*;
use crate::game;

pub struct App<'a> {
    window: Option<Window>,
    engine: Option<Engine<'a>>
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            window: None,
            engine: None
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_resizable(false)
                .with_title("Neothauma")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0)),
        ).unwrap();

        let window_ref: &'static Window = Box::leak(Box::new(window));
        self.window = Some(unsafe { std::ptr::read(window_ref) });

        let renderer = pollster::block_on(Renderer::new(window_ref));
        let ecs = ECS::new();

        self.engine = Some(Engine { renderer, ecs });

        if let Some(engine) = self.engine.as_mut() {
            load(engine);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().unwrap();
        let engine = self.engine.as_mut().unwrap();

        if id != window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                engine.renderer.resize(size);
                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = engine.renderer.render() {
                    eprintln!("Ошибка отрисовки: {e:?}");
                }

                window.request_redraw();
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.engine = None;
    }
}

fn load(engine: &mut Engine) {
    game::load::load(engine);
}
