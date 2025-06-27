use crate::engine::core::*;
use crate::engine::objects::*;
use crate::engine::transform::Transform;

pub fn load(engine: &mut Engine) {
    let mut cube = engine.create_entity(
        "Cube",
        create_cube(),
        Transform::default()
    );
}
