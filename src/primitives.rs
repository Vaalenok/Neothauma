#[derive(Debug, PartialEq)]
pub struct Point(pub f32, pub f32, pub f32);

impl Default for Point {
    fn default() -> Self {
        Point(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

pub trait Primitive {
    fn add_vector(&mut self, vector: &Vec3);
    fn subtract_vector(&mut self, vector: &Vec3);
}

pub trait IsPoint: Primitive {
    fn add_point(&mut self, point: &Point);
    fn subtract_point(&mut self, point: &Point);
    fn to_vec3(&self) -> Vec3;
    fn set(&mut self, point: &Point);
    fn draw(&self);
}

pub trait IsVec3: Primitive {
    fn to_point(&self) -> Point;
    fn set(&mut self, vector: &Vec3);
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
}

impl IsPoint for Point {
    fn add_point(&mut self, point: &Point) {
        self.0 += point.0;
        self.1 += point.1;
        self.2 += point.2;
    }

    fn subtract_point(&mut self, point: &Point) {
        self.0 -= point.0;
        self.1 -= point.1;
        self.2 -= point.2;
    }

    fn to_vec3(&self) -> Vec3 {
        Vec3(self.0, self.1, self.2)
    }

    fn set(&mut self, point: &Point) {
        self.0 = point.0;
        self.1 = point.1;
        self.2 = point.2;
    }

    fn draw(&self) {
        println!("Point: ({}, {}, {})", self.0, self.1, self.2);
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
}

impl IsVec3 for Vec3 {
    fn to_point(&self) -> Point {
        Point(self.0, self.1, self.2)
    }

    fn set(&mut self, vector: &Vec3) {
        self.0 = vector.0;
        self.1 = vector.1;
        self.2 = vector.2;
    }
}
