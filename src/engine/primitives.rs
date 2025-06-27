use std::ops::{Add, Sub, Mul, Div, Neg};

// Трейт отрисовки
pub trait Drawable {
    fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}

// Двухмерный вектор
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            *self / len
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn mul_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    pub fn div_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }

    pub fn lerp(&self, to: Self, t: f32) -> Self {
        *self * (1.0 - t) + to * t
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * (2.0 * self.dot(normal))
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

// Трёхмерный вектор
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
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

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            *self / len
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Четырёхмерный вектор
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            *self / len
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn mul_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar
        }
    }

    pub fn div_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar
        }
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.div_scalar(rhs)
    }
}

impl Neg for Vec4 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// Кватернион
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Quat {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            *self / len
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn inverse(&self) -> Self {
        let norm_sq = self.dot(*self);
        if norm_sq != 0.0 {
            self.conjugate() / norm_sq
        } else {
            *self
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn mul(&self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }

    pub fn rotate_vec3(&self, v: Vec3) -> Vec3 {
        let q_vec = Quat { x: v.x, y: v.y, z: v.z, w: 0.0 };
        let res = *self * q_vec * self.inverse();
        Vec3::new(res.x, res.y, res.z)
    }

    pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> Self {
        let half = angle_rad * 0.5;
        let sin = half.sin();
        let cos = half.cos();

        let axis_norm = axis.normalize();

        Self {
            x: axis_norm.x * sin,
            y: axis_norm.y * sin,
            z: axis_norm.z * sin,
            w: cos,
        }
    }

    pub fn lerp(&self, other: Self, t: f32) -> Self {
        ((*self) * (1.0 - t) + other * t).normalize()
    }

    pub fn slerp(&self, other: Self, t: f32) -> Self {
        let mut cos_theta = self.dot(other);
        let mut end = other;

        if cos_theta < 0.0 {
            cos_theta = -cos_theta;
            end = -other;
        }

        if cos_theta > 0.9995 {
            return self.lerp(end, t);
        }

        let theta = cos_theta.acos();
        let sin_theta = theta.sin();

        let w1 = ((1.0 - t) * theta).sin() / sin_theta;
        let w2 = (t * theta).sin() / sin_theta;

        (*self * w1 + end * w2).normalize()
    }

    pub fn to_mat3(&self) -> Mat3 {
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;

        let xx = self.x * x2;
        let yy = self.y * y2;
        let zz = self.z * z2;

        let xy = self.x * y2;
        let xz = self.x * z2;
        let yz = self.y * z2;

        let wx = self.w * x2;
        let wy = self.w * y2;
        let wz = self.w * z2;

        Mat3::new([
            [1.0 - (yy + zz), xy + wz, xz - wy],
            [xy - wz, 1.0 - (xx + zz), yz + wx],
            [xz + wy, yz - wx, 1.0 - (xx + yy)]
        ])
    }
}

impl Mul for Quat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Quat::mul(&self, rhs)
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Add for Quat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Quat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Quat {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Трёхмерная матрица
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    pub data: [[f32; 3]; 3]
}

impl Mat3 {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }

    pub const IDENTITY: Self = Self {
        data: [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ]
    };

    pub fn mul_vec3(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.data[0][0] * v.x + self.data[1][0] * v.y + self.data[2][0] * v.z,
            y: self.data[0][1] * v.x + self.data[1][1] * v.y + self.data[2][1] * v.z,
            z: self.data[0][2] * v.x + self.data[1][2] * v.y + self.data[2][2] * v.z
        }
    }

    pub fn mul_mat3(&self, rhs: &Mat3) -> Mat3 {
        let mut result = Mat3::default();

        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = 0.0;
                for k in 0..3 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        result
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat3(&rhs)
    }
}

// Четырёхмерная матрица
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }

    pub const IDENTITY: Self = Self {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    };

    pub fn from_transform(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let rot_mat = rotation.to_mat3();
        let mut data = [[0.0; 4]; 4];

        for i in 0..3 {
            data[i][0] = rot_mat.data[i][0] * scale.x;
            data[i][1] = rot_mat.data[i][1] * scale.y;
            data[i][2] = rot_mat.data[i][2] * scale.z;
            data[i][3] = 0.0;
        }

        data[3][0] = translation.x;
        data[3][1] = translation.y;
        data[3][2] = translation.z;
        data[3][3] = 1.0;

        Self { data }
    }

    pub fn mul_vec4(&self, v: Vec4) -> Vec4 {
        Vec4 {
            x: self.data[0][0] * v.x + self.data[1][0] * v.y + self.data[2][0] * v.z + self.data[3][0] * v.w,
            y: self.data[0][1] * v.x + self.data[1][1] * v.y + self.data[2][1] * v.z + self.data[3][1] * v.w,
            z: self.data[0][2] * v.x + self.data[1][2] * v.y + self.data[2][2] * v.z + self.data[3][2] * v.w,
            w: self.data[0][3] * v.x + self.data[1][3] * v.y + self.data[2][3] * v.z + self.data[3][3] * v.w
        }
    }

    pub fn mul_mat4(&self, rhs: &Mat4) -> Mat4 {
        let mut result = Mat4::default();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        result
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat4(&rhs)
    }
}

// Полигональная сетка
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new()
        }   
    }
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
}
