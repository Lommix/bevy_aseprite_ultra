#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var t: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var s: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var<uniform> texture_min: vec2u;
@group(#{MATERIAL_BIND_GROUP}) @binding(103) var<uniform> texture_max: vec2u;
@group(#{MATERIAL_BIND_GROUP}) @binding(104) var<uniform> time: f32;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    let size = textureDimensions(t);
    let uv_min = vec2f(texture_min) / vec2f(size);
    let uv_max = vec2f(texture_max) / vec2f(size);
    let uv = mix(uv_min, uv_max, in.uv);
    var col = textureSample(t, s, uv);
    col = max(col, vec4f(0.02, 0.02, 0.02, 0.0));

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    pbr_input.material.base_color *= col;

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);
    
    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // // we can optionally modify the final result here
    // out.color = out.color * 2.0;

    return out;
}
