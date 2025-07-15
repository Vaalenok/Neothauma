struct Uniforms {
    model: mat4x4<f32>,
    light_view_proj: mat4x4<f32>
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>
};

@vertex
fn vs_main(input: VertexInput) -> @builtin(position) vec4<f32> {
    let world_pos = uniforms.model * vec4(input.position, 1.0);
    let light_clip_pos = uniforms.light_view_proj * world_pos;
    return light_clip_pos;
}
