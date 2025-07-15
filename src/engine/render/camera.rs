use crate::engine::core::primitives::*;

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

impl Default for Camera {
    fn default() -> Self {
        // TODO: исправить
        Self::new(
            Vec3::new(-1.52, 3.77, 1.55),
            Quat::new(-0.44, -0.34, -0.19, 0.81),
            90.0,
            0.1,
            100.0
        )
    }
}

impl Camera {
    pub fn new(
        position: Vec3,
        rotation: Quat,
        fov: f32,
        near: f32,
        far: f32
    ) -> Self {
        Self {
            position,
            rotation,
            fov,
            near,
            far
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let rot = self.rotation.conjugate().to_mat3();
        let pos = -self.position;

        let tx = rot.data[0][0] * pos.x + rot.data[0][1] * pos.y + rot.data[0][2] * pos.z;
        let ty = rot.data[1][0] * pos.x + rot.data[1][1] * pos.y + rot.data[1][2] * pos.z;
        let tz = rot.data[2][0] * pos.x + rot.data[2][1] * pos.y + rot.data[2][2] * pos.z;

        Mat4::new([
            [rot.data[0][0],  rot.data[1][0],  rot.data[2][0],  0.0],
            [rot.data[0][1],  rot.data[1][1],  rot.data[2][1],  0.0],
            [rot.data[0][2],  rot.data[1][2],  rot.data[2][2],  0.0],
            [tx,              ty,              tz,              1.0]
        ])
    }

    pub fn get_projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective(self.fov, aspect, self.near, self.far)
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }

    pub fn move_right(&mut self, dist: f32) {
        self.position = self.position + self.right() * dist;
    }

    pub fn move_up(&mut self, dist: f32) {
        self.position = self.position + self.up() * dist;
    }

    pub fn move_forward(&mut self, dist: f32) {
        self.position = self.position + self.forward() * dist;
    }

    pub fn rotate_pitch(&mut self, angle_rad: f32) {
        let pitch = Quat::from_axis_angle(self.right(), angle_rad);
        self.rotation = (pitch * self.rotation).normalize();
    }
    
    pub fn rotate_yaw(&mut self, angle_rad: f32) {
        let yaw = Quat::from_axis_angle(self.up(), angle_rad);
        self.rotation = (yaw * self.rotation).normalize();
    }

    pub fn rotate_roll(&mut self, angle_rad: f32) {
        let roll = Quat::from_axis_angle(self.forward(), angle_rad);
        self.rotation = (roll * self.rotation).normalize();
    }
}
