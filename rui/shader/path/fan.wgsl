struct Globals {
    width_height: u32,
    aspect_ratio: f32,
}
struct VertexInput {
    @location(0) pos: vec2<f32>
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>
}

@group(0) @binding(0) var<uniform> globals: Globals;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    let pos = vec2(model.pos.x / globals.aspect_ratio, model.pos.y);

    var out: VertexOutput;
    out.pos = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}