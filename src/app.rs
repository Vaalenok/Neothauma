use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
use crate::engine::core::Engine;

pub struct App<'a> {
    engine: Option<Engine<'a>>
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self { engine: None }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_resizable(false)
                .with_title("Neothauma")
                .with_inner_size(LogicalSize::new(1280, 720))
        ).unwrap();

        let window = Arc::new(window);
        let engine = Engine::new(window.clone());

        self.engine = Some(engine);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(engine) = &mut self.engine {
            if Arc::ptr_eq(&engine.window, &engine.window) && engine.window.id() == id {
                match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::Resized(size) => {
                        engine.renderer.resize(size);
                        engine.window.request_redraw();
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        engine.window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        if let Err(e) = engine.renderer.render() {
                            eprintln!("Ошибка отрисовки: {:?}", e);
                        }
                        engine.window.request_redraw();
                    }
                    _ => {}
                }
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.engine = None;
    }
}
