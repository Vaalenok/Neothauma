mod utils;
mod engine;
mod app;

use winit::event_loop::{ControlFlow, EventLoop};
use app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
