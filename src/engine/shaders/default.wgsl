struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    light_pos: vec3<f32>,
    _pad: f32,
    camera_pos: vec3<f32>
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>
};

struct Light {
    position: vec3<f32>,
    _padding1: f32,
    color: vec3<f32>,
    intensity: f32,
};

@group(0) @binding(1)
var<uniform> light: Light;

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let model_pos = uniforms.model * vec4(input.position, 1.0);
    let model_normal = (uniforms.model * vec4(input.normal, 0.0)).xyz;

    output.clip_pos = uniforms.projection * uniforms.view * model_pos;
    output.world_pos = model_pos.xyz;
    output.normal = normalize(model_normal);

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let light_dir = normalize(light.position - input.world_pos);

    let view_dir = normalize(uniforms.camera_pos - input.world_pos);

    let halfway_dir = normalize(light_dir + view_dir);

    let ambient = 0.1;

    let diff = max(dot(normal, light_dir), 0.0);

    let spec = pow(max(dot(normal, halfway_dir), 0.0), 32.0);

    let light_color = light.color * light.intensity;
    let brightness = ambient + diff + 0.5 * spec;
    let color = light_color * brightness;

    return vec4(color, 1.0);
}
