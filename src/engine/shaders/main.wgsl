@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> lights: array<Light>;
@group(0) @binding(2) var<storage, read> light_count: LightCount;
@group(0) @binding(3) var depth_texture: texture_depth_cube;
@group(0) @binding(4) var depth_sampler: sampler_comparison;

struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    normal: mat4x4<f32>,
    camera_pos: vec3<f32>,
    _padding1: f32,
    light_pos: vec3<f32>,
    light_far_plane: f32,
    light_view_projection: mat4x4<f32>
};

struct Light {
    position: vec3<f32>,
    light_type: u32,
    color: vec3<f32>,
    intensity: f32,
    range: f32,
    _pad: array<f32, 6>
};

struct LightCount {
    count: u32
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) light_to_frag_vec: vec3<f32>
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let model_pos = uniforms.model * vec4(input.position, 1.0);
    out.world_pos = model_pos.xyz;
    out.normal = normalize((uniforms.normal * vec4(input.normal, 0.0)).xyz);

    out.clip_pos = uniforms.projection * uniforms.view * model_pos;
    out.light_to_frag_vec = out.world_pos - uniforms.light_pos;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let ambient = vec3(0.1);

    var lighting = ambient;

    for (var i: u32 = 0u; i < light_count.count; i++) {
        let light = lights[i];
        if (light.light_type != 0u) {
            continue;
        }

        let light_dir = normalize(light.position - input.world_pos);
        let dist_to_light = length(light.position - input.world_pos);
        if (dist_to_light > light.range) {
            continue;
        }

        let diffuse_strength = max(dot(input.normal, light_dir), 0.0);
        let attenuation = 1.0 / (dist_to_light * dist_to_light + 0.001);
        let diffuse = light.color * diffuse_strength * light.intensity * attenuation;

        let frag_to_light = input.world_pos - uniforms.light_pos;
        let current_depth = length(frag_to_light);
        let direction = normalize(frag_to_light);

        let bias = max(0.05 * (1.0 - dot(input.normal, -light_dir)), 0.01);

        let shadow = textureSampleCompare(
            depth_texture,
            depth_sampler,
            direction,
            (current_depth - bias) / uniforms.light_far_plane
        );

        lighting += diffuse * shadow;
    }

    return vec4(lighting, 1.0);
}
