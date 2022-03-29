struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0), interpolate(flat)]] color: vec4<f32>;
    [[location(1)]] tex_coordinates: vec2<f32>;
};

struct VertexInput {
    [[builtin(vertex_index)]] vid: u32;
};

struct InstanceInput {
    rect: vec4<f32>;
    color: vec4<f32>;
};

// Input the background texture from previous render pass
// @group(0) @binding(0) var t_background: texture2d<f32>;
// @group(0) @binding(1) var s_background: sampler;

// Input the image data
// Currently wgsl doesn't support texture arrays
// Therefore we draw all images individually

// Instance uniform buffer bind group
[[group(0), binding(0)]] var<uniform> instance: InstanceInput;

// Texture bind group
[[group(1), binding(0)]] var tex: texture_2d<f32>;
[[group(1), binding(1)]] var tex_sampler: sampler;


[[stage(vertex)]]
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    if (model.vid == 0u) {
        out.position = vec4<f32>(instance.rect.xy, 0.0, 1.0);
        out.tex_coordinates = vec2<f32>(0.0, 0.0);
    } else if (model.vid == 1u) {
        out.position = vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y, 0.0, 1.0);
        out.tex_coordinates = vec2<f32>(1.0, 0.0);
    } else if (model.vid == 2u) {
        out.position = vec4<f32>(instance.rect.x, instance.rect.y + instance.rect.w, 0.0, 1.0);
        out.tex_coordinates = vec2<f32>(0.0, 1.0);
    } else {
        out.position = vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y + instance.rect.w, 0.0, 1.0);
        out.tex_coordinates = vec2<f32>(1.0, 1.0);
    }
    out.color = instance.color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(tex, tex_sampler, in.tex_coordinates);
}