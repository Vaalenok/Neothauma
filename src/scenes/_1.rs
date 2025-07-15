use crate::engine::engine::*;
use crate::engine::objects::*;
use crate::engine::core::primitives::*;
use crate::engine::render::transform::*;

pub fn load(engine: &mut Engine) {
    light(engine);

    let floor = cube(engine);
    let cone = cone(engine, 32);
    let cylinder = cylinder(engine, 32);

    engine.transform(&floor, Transform::new(Vec3::Y * -0.5, Quat::IDENTITY, Vec3::new(100.0, 0.1, 100.0)));
    engine.transform(&cone, Transform::new(Vec3::X * 3.0, Quat::from_axis_angle(Vec3::X, 45.0), Vec3::IDENTITY));
    engine.transform(&cylinder, Transform::new(Vec3::Z * -3.0, Quat::IDENTITY, Vec3::IDENTITY));
}
