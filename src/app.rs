use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop},
    window::{Window, WindowId},
};
use crate::engine::renderer::Renderer;

pub struct App<'a> {
    window: Option<Window>,
    renderer: Option<Renderer<'a>>
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            window: None,
            renderer: None
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

        self.renderer = Some(pollster::block_on(Renderer::new(window_ref)));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().unwrap();
        let renderer = self.renderer.as_mut().unwrap();

        if id != window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                renderer.resize(size);
                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = renderer.render() {
                    eprintln!("Ошибка отрисовки: {e:?}");
                }

                window.request_redraw();
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer = None;
    }
}
