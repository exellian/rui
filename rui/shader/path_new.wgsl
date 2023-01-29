struct Globals {
    width_height: u32,
    aspect_ratio: f32,
}

struct PathSegment {
    typ: u32,
    flags: u32,
    rect_lu: vec2<f32>,
    rect_rl: vec2<f32>,
    param0: vec2<f32>,
    param1: vec2<f32>,
    param2: vec2<f32>,
    param3: vec2<f32>
}

struct Paths {
    segments: array<PathSegment>
}

struct VertexInput {
    @builtin(vertex_index) vid: u32
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
    @location(1) @interpolate(linear) norm_position: vec2<f32>,
    @location(2) @interpolate(flat) segment_range: vec2<u32>
}

struct Instance {
    @location(0) rect: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) segment_range: vec2<u32>
}

let SEGMENT_TYPE_LINEAR: u32 = 0u;
let SEGMENT_TYPE_ARC: u32 = 1u;
let SEGMENT_TYPE_QUADRATIC_BEZIER: u32 = 2u;
let SEGMENT_TYPE_CUBIC_BEZIER: u32 = 3u;
//let SEGMENT_TYPE_CATMULL_ROM: u32 = 4u;

let PI: f32 = 3.14159265358979323846264338327950288;

// Globals
@group(0) @binding(0) var<uniform> globals: Globals;
// A storage buffer, for the path segments
@group(1) @binding(0) var<storage, read> all_paths: Paths;

// coordinate system conversion
fn cc(pos: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(pos.x * 2.0 - 1.0, -2.0 * pos.y + 1.0, pos.z, pos.w);
    //return vec4<f32>(pos.x, pos.y, pos.z, posg.w);
}

fn unpack(x: u32) -> vec2<u32> {
    return vec2<u32>(x >> 16u, x & 0xFFFFu);
}

@vertex
fn vs_main(model: VertexInput, instance: Instance) -> VertexOutput {
    var vertex_out: VertexOutput;
    if (model.vid == 0u || model.vid == 3u) {
        vertex_out.position = cc(vec4<f32>(instance.rect.x, instance.rect.y * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.xy);
    } else if (model.vid == 2u || model.vid == 4u) {
        vertex_out.position = cc(vec4<f32>(instance.rect.z, instance.rect.w * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.z, instance.rect.w);
    } else if (model.vid == 1u) {
        vertex_out.position =  cc(vec4<f32>(instance.rect.x, instance.rect.w * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.x, instance.rect.w);
    } else {
        vertex_out.position = cc(vec4<f32>(instance.rect.z, instance.rect.y * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.z, instance.rect.y);
    }
    vertex_out.color = instance.color;
    vertex_out.segment_range = instance.segment_range;


    return vertex_out;
}


// ########## Hilfsfunktionen ##########

fn linearf(x: f32, m: f32, t: f32) -> f32 {
    return m * x + t;
}

@fragment
fn fs_main(vertex_out: VertexOutput) -> @location(0) vec4<f32> {

    let background: vec3<f32> = vec3<f32>(0.2);
    var y_closest_top: EdgePoint = EP_NONE;
    var y_closest_bot: EdgePoint = EP_NONE;
    var x_closest_left: EdgePoint = EP_NONE;
    var x_closest_right: EdgePoint = EP_NONE;

    // Go through all path segments and find the closest edge values
    for (var i: u32 = vertex_out.segment_range.x; i < vertex_out.segment_range.y; i = i + 1u) {

        var segment: PathSegment = all_paths.segments[i];

        // If current x vertical line not intersecting with the segment bounding box the segment and
        // if current y horizontal line not intersecting with the segment bounding box skip the segment
        if ((vertex_out.norm_position.x < segment.rect_lu.x || vertex_out.norm_position.x > segment.rect_rl.x) && (vertex_out.norm_position.y < segment.rect_lu.y || vertex_out.norm_position.y > segment.rect_rl.y)) {
            continue;
        }

        if (segment.typ == SEGMENT_TYPE_LINEAR) {
            var x: EdgePoint;
            var y: EdgePoint;
            edge_points_linear(vertex_out.norm_position, &segment, &x, &y);
            process_closer_y(vertex_out.norm_position, &y_closest_top, &y, true);
            process_closer_y(vertex_out.norm_position, &y_closest_bot, &y, false);
            process_closer_x(vertex_out.norm_position, &x_closest_left, &x, true);
            process_closer_x(vertex_out.norm_position, &x_closest_right, &x, false);
            // TODO check closer than any other edge point
        } else if (segment.typ == SEGMENT_TYPE_ARC) {

        } else if (segment.typ == SEGMENT_TYPE_QUADRATIC_BEZIER) {

        } else if (segment.typ == SEGMENT_TYPE_CUBIC_BEZIER) {
            var x1: EdgePoint;
            var x2: EdgePoint;
            var x3: EdgePoint;
            var y1: EdgePoint;
            var y2: EdgePoint;
            var y3: EdgePoint;
            edge_points_cubic_bezier(vertex_out.norm_position, i, &segment, &x1, &x2, &x3, &y1, &y2, &y3);
            process_closer_y(vertex_out.norm_position, &y_closest_top, &y1, true);
            process_closer_y(vertex_out.norm_position, &y_closest_bot, &y1, false);
            process_closer_y(vertex_out.norm_position, &y_closest_top, &y2, true);
            process_closer_y(vertex_out.norm_position, &y_closest_bot, &y2, false);
            process_closer_y(vertex_out.norm_position, &y_closest_top, &y3, true);
            process_closer_y(vertex_out.norm_position, &y_closest_bot, &y3, false);
            process_closer_x(vertex_out.norm_position, &x_closest_left, &x1, true);
            process_closer_x(vertex_out.norm_position, &x_closest_right, &x1, false);
            process_closer_x(vertex_out.norm_position, &x_closest_left, &x2, true);
            process_closer_x(vertex_out.norm_position, &x_closest_right, &x2, false);
            process_closer_x(vertex_out.norm_position, &x_closest_left, &x3, true);
            process_closer_x(vertex_out.norm_position, &x_closest_right, &x3, false);
        } else {

        }
    }
    return vec4<f32>(background, 1.0);
}