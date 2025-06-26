mod tests;
mod utils;
mod engine;
mod app;

use winit::event_loop::EventLoop;
use app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
