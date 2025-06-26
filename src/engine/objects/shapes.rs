use crate::engine::objects::primitives::{Drawable, Point};

pub struct Line {
    pub start: Point,
    pub end: Point,
    pub thickness: u16
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Point::default(),
            end: Point::default(),
            thickness: u16::default()
        }
    }
}

impl Line {
    pub fn new(start: Point, end: Point, thickness: u16) -> Self {
        Self { start, end, thickness }
    }
}

impl Drawable for Line {
    fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        // TODO
    }
}
