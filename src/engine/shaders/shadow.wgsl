struct Uniforms {
    model: mat4x4<f32>,
    light_view_proj: mat4x4<f32>
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.light_view_proj * uniforms.model * vec4(input.position, 1.0);
    return out;
}

@fragment
fn fs_main() {}
