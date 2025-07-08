use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::engine::renderer::transform::*;

const CLEAN_EPSILON: f32 = 1e-3;

fn clean_f32(value: f32) -> f32 {
    if value.abs() < CLEAN_EPSILON {
        0.0
    } else {
        (value * 1000.0).round() / 1000.0
    }
}

// Двухмерный вектор
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

// impl Vec2 {
//     pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
// 
//     pub fn clean(&self) -> Self {
//         Self {
//             x: clean_f32(self.x),
//             y: clean_f32(self.y)
//         }
//     }
// 
//     pub fn new(x: f32, y: f32) -> Self {
//         Self { x, y }.clean()
//     }
// 
//     pub fn length(&self) -> f32 {
//         self.dot(*self).sqrt()
//     }
// 
//     pub fn length_squared(&self) -> f32 {
//         self.dot(*self)
//     }
// 
//     pub fn normalize(&self) -> Self {
//         let len = self.length();
//         if len != 0.0 {
//             *self / len
//         } else {
//             *self
//         }
//     }
// 
//     pub fn dot(&self, other: Self) -> f32 {
//         self.x * other.x + self.y * other.y
//     }
// 
//     pub fn mul_scalar(&self, scalar: f32) -> Self {
//         Self {
//             x: self.x * scalar,
//             y: self.y * scalar
//         }
//     }
// 
//     pub fn div_scalar(&self, scalar: f32) -> Self {
//         Self {
//             x: self.x / scalar,
//             y: self.y / scalar
//         }
//     }
// 
//     pub fn lerp(&self, to: Self, t: f32) -> Self {
//         *self * (1.0 - t) + to * t
//     }
// 
//     pub fn reflect(&self, normal: Self) -> Self {
//         *self - normal * (2.0 * self.dot(normal))
//     }
// }
// 
// impl Add for Vec2 {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self::new(self.x + rhs.x, self.y + rhs.y)
//     }
// }
// 
// impl Sub for Vec2 {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self {
//         Self::new(self.x - rhs.x, self.y - rhs.y)
//     }
// }
// 
// impl Mul<f32> for Vec2 {
//     type Output = Self;
//     fn mul(self, rhs: f32) -> Self {
//         (Self {
//             x: self.x * rhs,
//             y: self.y * rhs
//         }).clean()
//     }
// }
// 
// impl Div<f32> for Vec2 {
//     type Output = Self;
//     fn div(self, rhs: f32) -> Self {
//         (Self {
//             x: self.x / rhs,
//             y: self.y / rhs
//         }).clean()
//     }
// }
// 
// impl Neg for Vec2 {
//     type Output = Self;
//     fn neg(self) -> Self {
//         Self::new(-self.x, -self.y)
//     }
// }

// Трёхмерный вектор
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const IDENTITY: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
    
    pub fn clean(&self) -> Self {
        Self {
            x: clean_f32(self.x),
            y: clean_f32(self.y),
            z: clean_f32(self.z)
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        (Self { x, y, z }).clean()
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
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
        (Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }).clean()
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        (Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }).clean()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Четырёхмерный вектор
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Vec4 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub w: f32
// }
// 
// impl Vec4 {
//     pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
//     
//     pub fn clean(&self) -> Self {
//         Self {
//             x: clean_f32(self.x),
//             y: clean_f32(self.y),
//             z: clean_f32(self.z),
//             w: clean_f32(self.w)
//         }
//     }
// 
//     pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
//         (Self { x, y, z, w }).clean()
//     }
// 
//     pub fn length(&self) -> f32 {
//         self.dot(*self).sqrt()
//     }
// 
//     pub fn length_squared(&self) -> f32 {
//         self.dot(*self)
//     }
// 
//     pub fn normalize(&self) -> Self {
//         let len = self.length();
//         if len != 0.0 {
//             *self / len
//         } else {
//             *self
//         }
//     }
// 
//     pub fn dot(&self, other: Self) -> f32 {
//         self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
//     }
// }
// 
// impl Add for Vec4 {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
//     }
// }
// 
// impl Sub for Vec4 {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self {
//         Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
//     }
// }
// 
// impl Mul<f32> for Vec4 {
//     type Output = Self;
//     fn mul(self, rhs: f32) -> Self {
//         Self::new(
//             self.x * rhs,
//             self.y * rhs,
//             self.z * rhs,
//             self.w * rhs
//         )
//     }
// }
// 
// impl Div<f32> for Vec4 {
//     type Output = Self;
//     fn div(self, rhs: f32) -> Self {
//         Self::new(
//             self.x / rhs,
//             self.y / rhs,
//             self.z / rhs,
//             self.w / rhs
//         )
//     }
// }
// 
// impl Neg for Vec4 {
//     type Output = Self;
//     fn neg(self) -> Self {
//         Self::new(-self.x, -self.y, -self.z, -self.w)
//     }
// }

// Кватернион
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Quat {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

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

    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
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
            w: cos
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
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let xy = x * y;
        let xz = x * z;
        let yz = y * z;

        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        Mat3::new([
            [1.0 - 2.0 * yy - 2.0 * zz,  2.0 * xy - 2.0 * wz,        2.0 * xz + 2.0 * wy      ],
            [2.0 * xy + 2.0 * wz,        1.0 - 2.0 * xx - 2.0 * zz,  2.0 * yz - 2.0 * wx      ],
            [2.0 * xz - 2.0 * wy,        2.0 * yz + 2.0 * wx,        1.0 - 2.0 * xx - 2.0 * yy]
        ])
    }
}

impl Mul for Quat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w
        }
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
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
            w: self.w / rhs
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
            w: self.w + rhs.w
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
            w: self.w - rhs.w
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
            w: -self.w
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        let u = Vec3::new(self.x, self.y, self.z);
        let s = self.w;

        let cross1 = u.cross(v);
        let cross2 = u.cross(cross1);

        v + cross1 * (2.0 * s) + cross2 * 2.0
    }
}

// Трёхмерная матрица
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4]
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

    pub fn from_transform(transform: &Transform) -> Self {
        let rot = transform.rotation.to_mat3();

        Self {
            data: [
                [rot.data[0][0] * transform.scale.x,  rot.data[1][0] * transform.scale.y,  rot.data[2][0] * transform.scale.z,  transform.position.x],
                [rot.data[0][1] * transform.scale.x,  rot.data[1][1] * transform.scale.y,  rot.data[2][1] * transform.scale.z,  transform.position.y],
                [rot.data[0][2] * transform.scale.x,  rot.data[1][2] * transform.scale.y,  rot.data[2][2] * transform.scale.z,  transform.position.z],
                [0.0,                                 0.0,                                 0.0,                                 1.0                 ]
            ]
        }
    }

    pub fn to_uniform(&self) -> Self {
        let mut result = Mat4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[j][i] = self.data[i][j];
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
        let mut result = Mat4::default();

        for row in 0..4 {
            for col in 0..4 {
                result.data[row][col] =
                    self.data[row][0] * rhs.data[0][col] +
                        self.data[row][1] * rhs.data[1][col] +
                        self.data[row][2] * rhs.data[2][col] +
                        self.data[row][3] * rhs.data[3][col];
            }
        }

        result
    }
}
