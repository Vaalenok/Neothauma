use std::any::Any;
use crate::engine::primitives::Point;

// Камера
struct Camera {
    pub min_poses: Point,
    pub max_pos: Point
}

// Рендеринг
struct Renderer<'a> {
    pub camera: Camera,
    pub objects_in_view: Vec<&'a dyn Any>
}

impl<'a> Renderer<'a> {
    fn draw(&self) {
        // TODO: отрисовка
    }
}
