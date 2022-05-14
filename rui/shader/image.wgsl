struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0), interpolate(flat)]] color: vec4<f32>;
    [[location(1), interpolate(linear)]] norm_position: vec2<f32>;
    [[location(2)]] tex_coordinates: vec2<f32>;
};

struct VertexInput {
    [[builtin(vertex_index)]] vid: u32;
};

struct InstanceInput {
    rect: vec4<f32>;
    color: vec4<f32>;
    radii: vec4<f32>;
};

struct Globals {
    width_height: u32;
    aspect_ratio: f32;
};

// Input the background texture from previous render pass
// @group(0) @binding(0) var t_background: texture2d<f32>;
// @group(0) @binding(1) var s_background: sampler;

[[group(0), binding(0)]] var<uniform> globals: Globals;

// Input the image data
// Currently wgsl doesn't support texture arrays
// Therefore we draw all images individually
// Instance uniform buffer bind group
[[group(1), binding(0)]] var<uniform> instance: InstanceInput;

// Texture bind group
[[group(2), binding(0)]] var tex: texture_2d<f32>;
[[group(2), binding(1)]] var tex_sampler: sampler;

// coordinate system conversion
fn cc(pos: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(pos.x * 2.0 - 1.0, -2.0 * pos.y + 1.0, pos.z, pos.w);
    //return vec4<f32>(pos.x, pos.y, pos.z, posg.w);
}

[[stage(vertex)]]
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    if (model.vid == 0u || model.vid == 3u) {
        out.position = cc(vec4<f32>(instance.rect.xy, 0.0, 1.0));
        out.norm_position = vec2<f32>(0.0, 0.0);
        out.tex_coordinates = vec2<f32>(0.0, 0.0);
    } else if (model.vid == 2u || model.vid == 4u) {
        out.position = cc(vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y + instance.rect.w, 0.0, 1.0));
        out.norm_position = vec2<f32>(1.0, 1.0 / globals.aspect_ratio);
        out.tex_coordinates = vec2<f32>(1.0, 1.0);
    } else if (model.vid == 1u) {
        out.position = cc(vec4<f32>(instance.rect.x, instance.rect.y + instance.rect.w, 0.0, 1.0));
        out.norm_position = vec2<f32>(0.0, 1.0 / globals.aspect_ratio);
        out.tex_coordinates = vec2<f32>(0.0, 1.0);
    } else {
        out.position = cc(vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y, 0.0, 1.0));
        out.norm_position = vec2<f32>(1.0, 0.0);
        out.tex_coordinates = vec2<f32>(1.0, 0.0);
    }
    out.color = instance.color;
    return out;
}

fn square(v: f32) -> f32 {
    return v*v;
}

fn border_radii(in: VertexOutput, color: vec4<f32>) -> vec4<f32> {
    let transparent: vec4<f32> = vec4<f32>(0.0);
    // Upper left corner
    if (in.norm_position.x < instance.radii.x && in.norm_position.y < instance.radii.x) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(instance.radii.x, instance.radii.x);
        if (dot(c, c) > square(instance.radii.x)) {
            return transparent;
        }
    }
    // Upper right
    if (in.norm_position.x > (1.0 - instance.radii.y) && in.norm_position.y < instance.radii.y) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(1.0 - instance.radii.y, instance.radii.y);
        if (dot(c, c) > square(instance.radii.y)) {
            return transparent;
        }
    }
    // lower right
    if (in.norm_position.x < instance.radii.z && in.norm_position.y > (1.0 / globals.aspect_ratio - instance.radii.z)) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(instance.radii.z, (1.0 / globals.aspect_ratio - instance.radii.z));
        if (dot(c, c) > square(instance.radii.z)) {
            return transparent;
        }
    }
    // lower right
    if (in.norm_position.x > (1.0 - instance.radii.w) && in.norm_position.y > (1.0 / globals.aspect_ratio - instance.radii.w)) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(1.0 - instance.radii.w, (1.0 / globals.aspect_ratio - instance.radii.w));
        if (dot(c, c) > square(instance.radii.w)) {
            return transparent;
        }
    }
    return color;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return border_radii(in, textureSample(tex, tex_sampler, in.tex_coordinates));
}