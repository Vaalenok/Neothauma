use std::sync::Arc;
use winit::{
    application::*,
    dpi::*,
    event::*,
    event_loop::*,
    window::*,
};
use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::engine::engine::Engine;

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

        crate::scenes::_1::load(self.engine.as_mut().unwrap()); // TODO: Добавить менеджер сцен

        window.request_redraw();
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
                        if let Some(engine) = &mut self.engine {
                            engine.update(1.0 / 60.0);

                            if let Err(e) = engine.render() {
                                eprintln!("Ошибка отрисовки: {:?}", e);
                            }

                            engine.window.request_redraw();
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => { // TODO: Сделать управление отдельно
                        let keycode = event.physical_key;
                        
                        if event.state == ElementState::Pressed {
                            match keycode {
                                PhysicalKey::Code(KeyCode::KeyW) => { engine.renderer.camera.move_forward(0.5); }
                                PhysicalKey::Code(KeyCode::KeyS) => { engine.renderer.camera.move_forward(-0.5); }
                                PhysicalKey::Code(KeyCode::KeyD) => { engine.renderer.camera.move_right(0.5); }
                                PhysicalKey::Code(KeyCode::KeyA) => { engine.renderer.camera.move_right(-0.5); }
                                
                                PhysicalKey::Code(KeyCode::ArrowUp) => { engine.renderer.camera.rotate_pitch(0.2) }
                                PhysicalKey::Code(KeyCode::ArrowDown) => { engine.renderer.camera.rotate_pitch(-0.2) }
                                PhysicalKey::Code(KeyCode::ArrowLeft) => { engine.renderer.camera.rotate_yaw(0.2) }
                                PhysicalKey::Code(KeyCode::ArrowRight) => { engine.renderer.camera.rotate_yaw(-0.2) }
                                PhysicalKey::Code(KeyCode::KeyE) => { engine.renderer.camera.rotate_roll(0.2) }
                                PhysicalKey::Code(KeyCode::KeyQ) => { engine.renderer.camera.rotate_roll(-0.2) }
                                
                                PhysicalKey::Code(KeyCode::Space) => { engine.renderer.camera.move_up(0.5) }
                                PhysicalKey::Code(KeyCode::ShiftLeft) => { engine.renderer.camera.move_up(-0.5) }
                                
                                PhysicalKey::Code(KeyCode::Equal) => { engine.renderer.camera.fov += 1.0; }
                                PhysicalKey::Code(KeyCode::Minus) => { engine.renderer.camera.fov -= 1.0; }
                                _ => {}
                            }
                        }
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
