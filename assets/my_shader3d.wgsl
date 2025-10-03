#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var t: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var s: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> texture_min: vec2u;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> texture_max: vec2u;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> time: f32;

@fragment
fn fragment(
    v: VertexOutput,
) -> @location(0) vec4<f32> {
    let size = textureDimensions(t);
    let uv_min = vec2f(texture_min) / vec2f(size);
    let uv_max = vec2f(texture_max) / vec2f(size);
    let uv = mix(uv_min, uv_max, v.uv);
    var col = textureSample(t, s, uv);
    col.x += 0.1 * sin(1.2 * time);
    col.y += 0.1 * sin(3.0 * time + 0.5);
    col.z += 0.1 * sin(0.4 * time + 1.0);
    return col;
}
