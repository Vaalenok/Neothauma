use crate::engine::engine::*;
use crate::engine::objects::*;
use crate::engine::core::primitives::*;

pub fn load(engine: &mut Engine) {
    let floor = cube(engine);
    let cube1 = cube(engine);
    let cube2 = cube(engine);
    let cube3 = cube(engine);
    let cube4 = cube(engine);

    light(engine);

    if let Some(transform) = engine.ecs.transforms.get_mut(&floor) {
        transform.position = transform.position + Vec3::Y * -1.0;
        transform.scale = transform.scale + Vec3::Y * 0.1;
        transform.scale = transform.scale + Vec3::X * 100.0;
        transform.scale = transform.scale + Vec3::Z * 100.0;
    }

    if let Some(transform) = engine.ecs.transforms.get_mut(&cube1) {
        transform.position = transform.position + Vec3::Z * 4.0;
        transform.scale = transform.scale + Vec3::X * 3.0;
    }
    
    if let Some(transform) = engine.ecs.transforms.get_mut(&cube2) {
        transform.position = transform.position + Vec3::Z * -4.0;
        transform.scale = transform.scale + Vec3::X * 3.0;
    }

    if let Some(transform) = engine.ecs.transforms.get_mut(&cube3) {
        transform.position = transform.position + Vec3::X * 4.0;
        transform.scale = transform.scale + Vec3::Z * 3.0;
    }

    if let Some(transform) = engine.ecs.transforms.get_mut(&cube4) {
        transform.position = transform.position + Vec3::X * -4.0;
        transform.scale = transform.scale + Vec3::Z * 3.0;
    }
}
