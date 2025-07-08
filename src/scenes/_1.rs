use crate::engine::engine::*;
use crate::engine::objects::*;
use crate::engine::core::primitives::*;

pub fn load(engine: &mut Engine) {
    let cube1 = cube(engine);
    let cube2 = cube(engine);

    if let Some(transform) = engine.ecs.transforms.get_mut(&cube1) {
        transform.scale = transform.scale + Vec3::X * 5.0;
    }
    
    if let Some(transform) = engine.ecs.transforms.get_mut(&cube2) {
        transform.position = transform.position + Vec3::Z * 4.0;
    }
}
