struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0), interpolate(flat)]] color: vec4<f32>;
};

struct VertexInput {
    [[builtin(vertex_index)]] vid: u32;
};

struct InstanceInput {
    [[location(0)]] rect: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
};

// Input the background texture from previous render pass
// @group(0) @binding(0) var t_background: texture2d<f32>;
// @group(0) @binding(1) var s_background: sampler;

// coordinate system conversion
fn cc(pos: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(pos.x * 2.0 - 1.0, -2.0 * pos.y + 1.0, pos.z, pos.w);
    //return vec4<f32>(pos.x, pos.y, pos.z, pos.w);
}

// Drawing counter clockwise
[[stage(vertex)]]
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    if (model.vid == 0u || model.vid == 3u) {
        out.position = cc(vec4<f32>(instance.rect.xy, 0.0, 1.0));
    } else if (model.vid == 2u || model.vid == 4u) {
        out.position = cc(vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y + instance.rect.w, 0.0, 1.0));
    } else if (model.vid == 1u) {
        out.position = cc(vec4<f32>(instance.rect.x, instance.rect.y + instance.rect.w, 0.0, 1.0));
    } else {
        out.position = cc(vec4<f32>(instance.rect.x + instance.rect.z, instance.rect.y, 0.0, 1.0));
    }
    out.color = instance.color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}