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
    shadow_mat: mat4x4<f32>
};

struct Light {
    position: vec3<f32>,
    _pad1: f32,
    color: vec3<f32>,
    _pad2: f32,
    intensity: f32,
    _pad3: array<f32, 7>
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
    @location(2) frag_coord: vec2<f32>,
    @location(3) shadow_uv: vec4<f32>
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model_pos = uniforms.model * vec4(input.position, 1.0);
    let normal = (uniforms.normal * vec4(input.normal, 0.0)).xyz;
    let shadow_pos = uniforms.shadow_mat * model_pos;

    out.clip_pos = uniforms.projection * uniforms.view * model_pos;
    out.world_pos = model_pos.xyz;
    out.normal = normalize(normal);
    out.frag_coord = out.clip_pos.xy / out.clip_pos.w;
    out.shadow_uv = shadow_pos;

    return out;
}

fn pseudo_noise(coord: vec2<f32>) -> f32 {
    let p = fract(sin(dot(coord, vec2<f32>(12.9898,78.233))) * 43758.5453);
    return p;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var lit = 0.0;
    let ambient = 0.15;

    for (var i = 0u; i < light_count.count; i = i + 1u) {
        let light = lights[i];
        let light_dir = normalize(light.position - input.world_pos);
        lit += max(dot(input.normal, light_dir), 0.0) * light.intensity;
    }

    lit = clamp(lit, 0.0, 1.0);

    let light_pos = lights[0].position;
    let light_to_frag = input.world_pos - light_pos;
    let dist = length(light_to_frag);

    let shadow = textureSampleCompare(depth_texture, depth_sampler, light_to_frag, dist - 0.0005);

    let brightness = ambient + shadow * lit;
    let quantized = floor(brightness * 5.0) / 5.0;
    let noise = pseudo_noise(input.frag_coord * 300.0);
    let _final = quantized > noise;

    return vec4(vec3(f32(_final)), 1.0);
}
