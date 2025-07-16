use crate::engine::engine::*;
use crate::engine::objects::*;
use crate::engine::core::primitives::*;
use crate::engine::render::transform::*;

pub fn load(engine: &mut Engine) {
    let light = light(engine);
    engine.edit_light(&light, Vec3::IDENTITY, 10.0, 1000.0);

    let floor = cube(engine);
    engine.transform(&floor, Transform::new(Vec3::Y * -1.0, Quat::IDENTITY, Vec3::new(100.0, 0.1, 100.0)));

    let cone = cone(engine, 32);
    engine.transform(&cone, Transform::new(Vec3::new(3.0, 0.0, -3.0), Quat::from_axis_angle(Vec3::X, 45.0), Vec3::IDENTITY));

    let cylinder = cylinder(engine, 32);
    engine.transform(&cylinder, Transform::new(Vec3::new(0.0, 0.0, -3.0), Quat::IDENTITY, Vec3::IDENTITY));

    let sphere = sphere(engine, 32);
    engine.transform(&sphere, Transform::new(Vec3::new(-3.0, 0.0, -3.0), Quat::IDENTITY, Vec3::IDENTITY));
}
