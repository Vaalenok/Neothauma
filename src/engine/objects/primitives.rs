use crate::utils;

const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

// Трейт отрисовки 
pub trait Drawable {
    fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}

// Основа примитива
pub trait Primitive {
    fn add_vector(&mut self, vector: &Vec3);
    fn subtract_vector(&mut self, vector: &Vec3);
    fn set(&mut self, x: f32, y: f32, z: f32);
}

// Точка
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Primitive for Point {
    fn add_vector(&mut self, vector: &Vec3) {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;
    }

    fn subtract_vector(&mut self, vector: &Vec3) {
        self.x-= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
    }

    fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add_point(&mut self, point: &Point) {
        self.x += point.x;
        self.y += point.y;
        self.z += point.z;
    }

    pub fn subtract_point(&mut self, point: &Point) {
        self.x -= point.x;
        self.y -= point.y;
        self.z -= point.z;
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
}

impl Drawable for Point {
    fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        // TODO
    }
}

// Вектор
#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Primitive for Vec3 {
    fn add_vector(&mut self, vector: &Vec3) {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;
    }

    fn subtract_vector(&mut self, vector: &Vec3) {
        self.x -= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
    }

    fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_point(&self) -> Point {
        Point { x: self.x, y: self.y, z: self.z }
    }

    pub fn rotate_xy(&mut self, angle: f32) {
        let rad = angle * DEG_TO_RAD;
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        self.x = utils::round_to_threshold(x);
        self.y = utils::round_to_threshold(y);
    }

    pub fn rotate_yz(&mut self, angle: f32) {
        let rad = angle * DEG_TO_RAD;
        let cos = rad.cos();
        let sin = rad.sin();
        let y = self.y * cos - self.z * sin;
        let z = self.y * sin + self.z * cos;
        self.y = utils::round_to_threshold(y);
        self.z = utils::round_to_threshold(z);
    }

    pub fn rotate_xz(&mut self, angle: f32) {
        let rad = angle * DEG_TO_RAD;
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.x * cos - self.z * sin;
        let z = self.x * sin + self.z * cos;
        self.x = utils::round_to_threshold(x);
        self.z = utils::round_to_threshold(z);
    }

    pub fn scale(&mut self, factor: (f32, f32, f32)) {
        self.x *= factor.0;
        self.y *= factor.1;
        self.z *= factor.2;
    }
}

impl Drawable for Vec3 {
    fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        // TODO
    }
}
