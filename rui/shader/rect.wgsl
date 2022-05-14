struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0), interpolate(flat)]] color: vec4<f32>;
    [[location(1), interpolate(linear)]] norm_position: vec2<f32>;
    [[location(2), interpolate(flat)]] radii: vec4<f32>;
    [[location(3), interpolate(flat)]] ar: f32;
};

struct VertexInput {
    [[builtin(vertex_index)]] vid: u32;
};

struct InstanceInput {
    [[location(0)]] rect: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] radii: vec4<f32>;
};

struct Globals {
    width_height: u32;
    aspect_ratio: f32;
};

[[group(0), binding(0)]] var<uniform> globals: Globals;

// Input the background texture from previous render pass
// @group(0) @binding(0) var t_background: texture2d<f32>;
// @group(0) @binding(1) var s_background: sampler;

// coordinate system conversion
fn cc(pos: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(pos.x * 2.0 - 1.0, -2.0 * pos.y + 1.0, pos.z, pos.w);
    //return vec4<f32>(pos.x, pos.y, pos.z, posg.w);
}

// Drawing counter clockwise
[[stage(vertex)]]
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    if (model.vid == 0u || model.vid == 3u) {
        out.position = cc(vec4<f32>(instance.rect.xy, 0.0, 1.0));
        out.norm_position = vec2<f32>(0.0, 0.0);
    } else if (model.vid == 2u || model.vid == 4u) {
        out.position = cc(vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y + instance.rect.w, 0.0, 1.0));
        out.norm_position = vec2<f32>(1.0, 1.0 / globals.aspect_ratio);
    } else if (model.vid == 1u) {
        out.position = cc(vec4<f32>(instance.rect.x, instance.rect.y + instance.rect.w, 0.0, 1.0));
        out.norm_position = vec2<f32>(0.0, 1.0 / globals.aspect_ratio);
    } else {
        out.position = cc(vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y, 0.0, 1.0));
        out.norm_position = vec2<f32>(1.0, 0.0);
    }
    out.color = instance.color;
    out.radii = instance.radii;
    out.ar = instance.rect.x / instance.rect.y;
    return out;
}

fn square(v: f32) -> f32 {
    return v*v;
}

fn border_radii(in: VertexOutput, color: vec4<f32>) -> vec4<f32> {
    let transparent: vec4<f32> = vec4<f32>(0.0);
    // Upper left corner
    if (in.norm_position.x < in.radii.x && in.norm_position.y < in.radii.x) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(in.radii.x, in.radii.x);
        if (dot(c, c) > square(in.radii.x)) {
            return transparent;
        }
    }
    // Upper right
    if (in.norm_position.x > (1.0 - in.radii.y) && in.norm_position.y < in.radii.y) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(1.0 - in.radii.y, in.radii.y);
        if (dot(c, c) > square(in.radii.y)) {
            return transparent;
        }
    }
    // lower right
    if (in.norm_position.x < in.radii.z && in.norm_position.y > (1.0 / globals.aspect_ratio - in.radii.z)) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(in.radii.z, (1.0 / globals.aspect_ratio - in.radii.z));
        if (dot(c, c) > square(in.radii.z)) {
            return transparent;
        }
    }
    // lower right
    if (in.norm_position.x > (1.0 - in.radii.w) && in.norm_position.y > (1.0 / globals.aspect_ratio - in.radii.w)) {
        let c: vec2<f32> = in.norm_position - vec2<f32>(1.0 - in.radii.w, (1.0 / globals.aspect_ratio - in.radii.w));
        if (dot(c, c) > square(in.radii.w)) {
            return transparent;
        }
    }
    return color;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return border_radii(in, in.color);
}