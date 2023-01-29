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

let NO_SOLUTION: f32 = -1.0;
let GRADIENT_DIRECTION_X_Y: f32 = 0.0;
let GRADIENT_DIRECTION_X_INV_Y: f32 = 1.0;
let GRADIENT_DIRECTION_X_Y_INV: f32 = 2.0;
let GRADIENT_DIRECTION_X_INV_Y_INV: f32 = 3.0;

// Globals
@group(0) @binding(0) var<uniform> globals: Globals;
// A storage buffer, for the path segments
@group(1) @binding(0) var<storage, read> all_paths: Paths;
// A texture containing path segment solving results that got computed in a
// previous render pass
@group(2) @binding(0) var cubic_bezier_solutions_x: texture_2d<f32>;
@group(2) @binding(1) var cubic_bezier_area_directions_x: texture_2d<u32>;
@group(3) @binding(0) var cubic_bezier_solutions_y: texture_2d<f32>;
@group(3) @binding(1) var cubic_bezier_area_directions_y: texture_2d<u32>;


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

struct EdgePoint {
    position: vec2<f32>,
    // Inverted means that the area is not at the bottom or right and is instead on the top or left
    // So the default is: area is at BOTTOM and RIGHT
    inv: bool,
    is_set: bool,
}

let EP_NONE: EdgePoint = EdgePoint(vec2<f32>(0.0, 0.0), false, false);

// ########## Hilfsfunktionen ##########

fn linearf(x: f32, m: f32, t: f32) -> f32 {
    return m * x + t;
}

fn process_closer_x(pos: vec2<f32>, current_in_out: ptr<function, EdgePoint>, new_in: ptr<function, EdgePoint>, left: bool) {//, x_axis_inv: bool) {
    if (!(*new_in).is_set) {
        return;
    }
    var x_diff: f32 = (*new_in).position.x - pos.x;

    // If the new point is on the right side
    if ((x_diff > 0.0 && left) || (x_diff < 0.0 && !left)) {

        if (!(*current_in_out).is_set) {
            (*current_in_out) = (*new_in);
        } else {
            let x_old_distance: f32 = abs((*current_in_out).position.x - pos.x);
            let x_new_distance: f32 = abs(x_diff);
            if (x_new_distance < x_old_distance) {
                (*current_in_out) = (*new_in);
            }
        }
    }
}

fn process_closer_y(pos: vec2<f32>, current_in_out: ptr<function, EdgePoint>, new_in: ptr<function, EdgePoint>, top: bool) {
    if (!(*new_in).is_set) {
        return;
    }
    var y_diff: f32 = (*new_in).position.y - pos.y; // -1 because the y_axis is inverted

    // If the new point is on the right side
    if ((y_diff > 0.0 && top) || (y_diff < 0.0 && !top)) {

        if (!(*current_in_out).is_set) {
            (*current_in_out) = (*new_in);
        } else {
            let y_old_distance: f32 = abs((*current_in_out).position.y - pos.y);
            let y_new_distance: f32 = abs(y_diff);
            if (y_new_distance < y_old_distance) {
                (*current_in_out) = (*new_in);
            }
        }
    }
}

// ########## Main solving functions ##########

fn edge_points_linear(
    // current position in the svg view
    pos: vec2<f32>,
    // linear segment
    segment: ptr<function, PathSegment>,
    // EdgePoint in x direction for the current y position
    x_out: ptr<function, EdgePoint>,
    // EdgePoint in y direction for the current x position
    y_out: ptr<function, EdgePoint>
) {
    var delta: vec2<f32> = (*segment).param1 - (*segment).param0;

    (*x_out) = EP_NONE;
    (*y_out) = EP_NONE;

    if (delta.x == 0.0 && delta.y != 0.0) {
        // segment is verical line
        // no y solution

        // |
        // |----p
        // |

        (*x_out).position = vec2<f32>((*segment).param0.x, pos.y);
        if (delta.y < 0.0) {
            (*x_out).inv = true;
        }
        (*x_out).is_set = true;
    } else if (delta.y == 0.0 && delta.x != 0.0) {
        // segment is horizontal line
        // no x solution

        // ____________
        //      |
        //      |
        //      p
        (*y_out).position = vec2<f32>(pos.x, (*segment).param0.y);
        if (delta.x >= 0.0) {
            (*y_out).inv = true;
        }
        (*y_out).is_set = true;
    } else if (delta.y != 0.0 && delta.x != 0.0) {

        // This is the normal case where we have a oblique line
        //                    x
        //       /            |
        //      /|            |
        //     / |            |
        //    /  |            |
        //   /---p_1          |
        //  /----------------p_2

        // p_2 doesn't have a intersection point with the line in y direction but in x direction (keep that case in mind and vice versa)

        let grady = delta.x / delta.y;
        let gradx = 1.0 / grady; // Note: this is equals delta.y / delta.x

        // We have to equations for the line:

        // y(x) = grady * x + ty  <=> y(x) - grady * x = ty
        // x(y) = gradx * y + tx  <=> x(y) - gradx * y = tx

        let ty = (*segment).param0.y - grady * (*segment).param0.x;
        let tx = (*segment).param0.x - gradx * (*segment).param0.y;

        // In this case we have a y-solution
        if (pos.x >= (*segment).rect_lu.x && pos.x <= (*segment).rect_rl.x) {
            (*y_out).position = vec2<f32>(pos.x, grady * pos.x + ty);
            if (delta.x >= 0.0) {
                (*y_out).inv = true;
            }
            (*y_out).is_set = true;
        }

        // In this case we have a x-solution
        if (pos.y >= (*segment).rect_lu.y && pos.y <= (*segment).rect_rl.y) {
            (*x_out).position = vec2<f32>(gradx * pos.y + tx, pos.y);
            if (delta.y < 0.0) {
                (*x_out).inv = true;
            }
            (*x_out).is_set = true;
        }
    } else {

        // both delta.x and delta.y are equal to 0
        // In this case the line is a point and we simply ignore it


        //
        //      x
        //
        //
        //          p
    }
}

fn edge_points_cubic_bezier(
    // current position in the svg view
    pos: vec2<f32>,
    segment_index: u32,
    segment: ptr<function, PathSegment>,
    x1: ptr<function, EdgePoint>,
    x2: ptr<function, EdgePoint>,
    x3: ptr<function, EdgePoint>,
    y1: ptr<function, EdgePoint>,
    y2: ptr<function, EdgePoint>,
    y3: ptr<function, EdgePoint>
) {
    (*x1) = EP_NONE;
    (*x2) = EP_NONE;
    (*x3) = EP_NONE;
    (*y1) = EP_NONE;
    (*y2) = EP_NONE;
    (*y3) = EP_NONE;

    let extent: vec2<u32> = unpack(globals.width_height);
    let segment_extent: vec2<f32> = (*segment).rect_rl - (*segment).rect_lu;

    // For cubic beziers we actually do not solve anything in the fragment shader
    // we already did that in a compute shader prepass.
    // The results are stored in 4 textures

    if (pos.y >= (*segment).rect_lu.y || pos.y <= (*segment).rect_rl.y) {
        let index_x: u32 = u32((pos.y - (*segment).rect_lu.y) * f32(extent.y));
        let solutions_x: vec4<f32> = textureLoad(cubic_bezier_solutions_x, vec2<i32>(i32(index_x), i32(segment_index)), 0);
        let area_directions_x: vec4<u32> = textureLoad(cubic_bezier_area_directions_x, vec2<i32>(i32(index_x), i32(segment_index)), 0);

        if (solutions_x.x > NO_SOLUTION) {
            (*x1).position = vec2<f32>(solutions_x.x, solutions_x.w);
            (*x1).inv = area_directions_x.x == 1u;
            (*x1).is_set = true;
        }
        if (solutions_x.y > NO_SOLUTION) {
            (*x2).position = vec2<f32>(solutions_x.y, solutions_x.w);
            (*x2).inv = area_directions_x.y == 1u;
            (*x2).is_set = true;
        }
        if (solutions_x.z > NO_SOLUTION) {
            (*x2).position = vec2<f32>(solutions_x.z, solutions_x.w);
            (*x2).inv = area_directions_x.z == 1u;
            (*x2).is_set = true;
        }
    }

    if (pos.x >= (*segment).rect_lu.x || pos.x <= (*segment).rect_rl.x) {
        let index_y: u32 = u32((pos.x - (*segment).rect_lu.x) * f32(extent.x));
        let solutions_y: vec4<f32> = textureLoad(cubic_bezier_solutions_y, vec2<i32>(i32(index_y), i32(segment_index)), 0);
        let area_directions_y: vec4<u32> = textureLoad(cubic_bezier_area_directions_y, vec2<i32>(i32(index_y), i32(segment_index)), 0);

        if (solutions_y.x > NO_SOLUTION) {
            (*y1).position = vec2<f32>(solutions_y.w, solutions_y.x);
            (*y1).inv = area_directions_y.x == 1u;
            (*y1).is_set = true;
        }
        if (solutions_y.y > NO_SOLUTION) {
            (*y2).position = vec2<f32>(solutions_y.w, solutions_y.y);
            (*y2).inv = area_directions_y.y == 1u;
            (*y2).is_set = true;
        }
        if (solutions_y.z > NO_SOLUTION) {
            (*y3).position = vec2<f32>(solutions_y.w, solutions_y.z);
            (*y3).inv = area_directions_y.z == 1u;
            (*y3).is_set = true;
        }
    }
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

    if (y_closest_top.is_set && !y_closest_top.inv ) {//  && !y_closest_top.inv && y_closest_bot.inv) {
        return vec4<f32>(0.0, 0.0, 1.0, 1.0);
    }

    if (x_closest_left.is_set && x_closest_right.is_set && y_closest_top.is_set && y_closest_bot.is_set) {
        //return vec4<f32>(0.0, 0.0, 1.0, 1.0);
//return vec4<f32>(0.0, 0.0, 1.0, 1.0);
        if (y_closest_top.inv && y_closest_bot.inv) {
            //return vec4<f32>(0.0, 0.0, 1.0, 1.0);
        }
    }
    return vec4<f32>(background, 1.0);
}