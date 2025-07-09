use crate::engine::engine::*;
use crate::engine::objects::*;
use crate::engine::core::primitives::*;
use crate::engine::render::transform::*;

pub fn load(engine: &mut Engine) {
    let floor = cube(engine);
    
    let cube1 = cube(engine);
    let cube2 = cube(engine);
    let cube3 = cube(engine);
    let cube4 = cube(engine);
    
    let test_cube = cube(engine);

    let light_entity = light(engine);
    engine.transform(&light_entity, Transform::new(Vec3::new(0.0, 5.0, 0.0), Quat::ZERO, Vec3::ZERO));
    
    engine.transform(&floor, Transform::new(Vec3::Y * -1.0, Quat::IDENTITY, Vec3::new(100.0, 0.1, 100.0)));
    engine.transform(&cube1, Transform::new(Vec3::Z * -4.0, Quat::IDENTITY, Vec3::IDENTITY + Vec3::X * 2.0));
    engine.transform(&cube2, Transform::new(Vec3::X * 4.0, Quat::IDENTITY, Vec3::IDENTITY + Vec3::Z * 2.0));
    engine.transform(&cube3, Transform::new(Vec3::X * -4.0, Quat::IDENTITY, Vec3::IDENTITY + Vec3::Z * 2.0));
    engine.transform(&cube4, Transform::new(Vec3::Z * 4.0, Quat::IDENTITY, Vec3::IDENTITY + Vec3::X * 2.0));
    engine.transform(&test_cube, Transform::new(Vec3::Z * 6.0, Quat::IDENTITY, Vec3::IDENTITY + Vec3::X * 2.0));
}
