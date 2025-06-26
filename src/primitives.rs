use crate::utils;

const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

// Примитив
pub trait Primitive {
    fn add_vector(&mut self, vector: &Vec3);
    fn subtract_vector(&mut self, vector: &Vec3);
    fn set(&mut self, values: (f32, f32, f32));
}

// Точка
#[derive(Debug, PartialEq)]
pub struct Point(pub f32, pub f32, pub f32);

impl Default for Point {
    fn default() -> Self {
        Point(0.0, 0.0, 0.0)
    }
}

impl Primitive for Point {
    fn add_vector(&mut self, vector: &Vec3) {
        self.0 += vector.0;
        self.1 += vector.1;
        self.2 += vector.2;
    }

    fn subtract_vector(&mut self, vector: &Vec3) {
        self.0 -= vector.0;
        self.1 -= vector.1;
        self.2 -= vector.2;
    }

    fn set(&mut self, values: (f32, f32, f32)) {
        self.0 = values.0;
        self.1 = values.1;
        self.2 = values.2;
    }
}

impl Point {
    pub fn add_point(&mut self, point: &Point) {
        self.0 += point.0;
        self.1 += point.1;
        self.2 += point.2;
    }

    pub fn subtract_point(&mut self, point: &Point) {
        self.0 -= point.0;
        self.1 -= point.1;
        self.2 -= point.2;
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3(self.0, self.1, self.2)
    }

    pub fn draw(&self) {
        println!("Point: ({}, {}, {})", self.0, self.1, self.2);
    }
}

// Вектор
#[derive(Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Primitive for Vec3 {
    fn add_vector(&mut self, vector: &Vec3) {
        self.0 += vector.0;
        self.1 += vector.1;
        self.2 += vector.2;
    }

    fn subtract_vector(&mut self, vector: &Vec3) {
        self.0 -= vector.0;
        self.1 -= vector.1;
        self.2 -= vector.2;
    }

    fn set(&mut self, values: (f32, f32, f32)) {
        self.0 = values.0;
        self.1 = values.1;
        self.2 = values.2;
    }
}

impl Vec3 {
    pub fn to_point(&self) -> Point {
        Point(self.0, self.1, self.2)
    }

    pub fn rotate_xy(&mut self, angle: f32) {
        let rad = angle * DEG_TO_RAD;
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.0 * cos - self.1 * sin;
        let y = self.0 * sin + self.1 * cos;
        self.0 = utils::round_to_n(x, 2);
        self.1 = utils::round_to_n(y, 2);
    }

    pub fn rotate_yz(&mut self, angle: f32) {
        let rad = angle * DEG_TO_RAD;
        let cos = rad.cos();
        let sin = rad.sin();
        let y = self.1 * cos - self.2 * sin;
        let z = self.1 * sin + self.2 * cos;
        self.1 = utils::round_to_n(y, 2);
        self.2 = utils::round_to_n(z, 2);
    }

    pub fn rotate_xz(&mut self, angle: f32) {
        let rad = angle * DEG_TO_RAD;
        let cos = rad.cos();
        let sin = rad.sin();
        let x = self.0 * cos - self.2 * sin;
        let z = self.0 * sin + self.2 * cos;
        self.0 = utils::round_to_n(x, 2);
        self.2 = utils::round_to_n(z, 2);
    }

    pub fn scale(&mut self, factor: (f32, f32, f32)) {
        self.0 *= factor.0;
        self.1 *= factor.1;
        self.2 *= factor.2;
    }
}
