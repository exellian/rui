struct Globals {
    width_height: u32;
    aspect_ratio: f32;
};


struct PathSegment {
    typ: u32;
    flags: u32;
    rect_lu: vec2<f32>;
    rect_rl: vec2<f32>;
    param0: vec2<f32>;
    param1: vec2<f32>;
    param2: vec2<f32>;
    param3: vec2<f32>;
};

struct Paths {
    segments: array<PathSegment>;
};

struct VertexInput {
    [[builtin(vertex_index)]] vid: u32;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0), interpolate(flat)]] color: vec4<f32>;
    [[location(1), interpolate(linear)]] norm_position: vec2<f32>;
    [[location(2), interpolate(flat)]] segment_range: vec2<u32>;
};

struct Instance {
    [[location(0)]] rect: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] segment_range: vec2<u32>;
};

struct EdgeValue {
    y: f32;
    top_is_area: bool;
    is_set: bool;
};

let SEGMENT_TYPE_LINEAR: u32 = 0u;
let SEGMENT_TYPE_ARC: u32 = 1u;
let SEGMENT_TYPE_QUADRATIC_BEZIER: u32 = 2u;
let SEGMENT_TYPE_CUBIC_BEZIER: u32 = 3u;
//let SEGMENT_TYPE_CATMULL_ROM: u32 = 4u;

let PI: f32 = 3.14159265358979323846264338327950288;


// Globals
[[group(0), binding(0)]] var<uniform> globals: Globals;
// A storage buffer, for the path segments
[[group(1), binding(0)]] var<storage, read> all_paths: Paths;
// A texture containing path segment solving results that got computed in a
// previous render pass
[[group(2), binding(0)]] var compute_texture: texture_2d<f32>;

let ev_none: EdgeValue = EdgeValue(0.0, false, false);

fn ev_some(y: f32, top_is_area: bool) -> EdgeValue {
    var x: EdgeValue;
    x.y = y;
    x.top_is_area = top_is_area;
    x.is_set = true;
    return x;
}
fn ev_check_closer_top(self_ev: ptr<function, EdgeValue>, other: ptr<function, EdgeValue>, pos: vec2<f32>) {
    if (!(*other).is_set) {
        return;
    }
    let distance: f32 = (*other).y - (*self_ev).y;

    if ((!(*self_ev).is_set && (*other).y < pos.y) || ((*other).y < pos.y && distance >= 0.0)) {
        (*self_ev) = (*other);
    }
}
fn ev_check_closer_bot(self_ev: ptr<function, EdgeValue>, other: ptr<function, EdgeValue>, pos: vec2<f32>) {
    if (!(*other).is_set) {
        return;
    }
    let distance: f32 = (*other).y - (*self_ev).y;

    if ((!(*self_ev).is_set && (*other).y > pos.y) || ((*other).y > pos.y && distance >= 0.0)) {
        (*self_ev) = (*other);
    }
}

// coordinate system conversion
fn cc(pos: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(pos.x * 2.0 - 1.0, -2.0 * pos.y + 1.0, pos.z, pos.w);
    //return vec4<f32>(pos.x, pos.y, pos.z, posg.w);
}

fn unpack(x: u32) -> vec2<u32> {
    return vec2<u32>(x >> 16u, x & 0xFFFFu);
}

fn rect_width(rect_lu: vec2<f32>, rect_rl: vec2<f32>) -> f32 {
    return rect_rl.x - rect_lu.x;
}

fn edge_values_cubic_bezier(
    pos: vec2<f32>,
    segment_index: u32,
    segment: ptr<function, PathSegment>,
    x: ptr<function, EdgeValue>,
    y: ptr<function, EdgeValue>,
    z: ptr<function, EdgeValue>
) {

    // Get the width and height of the screen
    let extent: vec2<u32> = unpack(globals.width_height);
    let segment_width: f32 = rect_width((*segment).rect_lu, (*segment).rect_rl);
    //let absolute_segment_width: u32 = u32(round(segment_width * f32(extent.x)));
    let index: u32 = u32((pos.x - (*segment).rect_lu.x) * f32(extent.x));
    let solution: vec4<f32> = textureLoad(compute_texture, vec2<i32>(i32(index), i32(segment_index)), 0);
    var solution_count: u32 = u32(round(solution.w));

    //var _1: bool = false;
    //var _2: bool = false;
    //var _3: bool = false;
    //var solution_count: u32 = 0u;
    //let solution: vec3<f32> = solve_cubic_bezier(
    //    pos.x,
    //    (*segment).param0,
    //    (*segment).param1,
    //    (*segment).param2,
    //    (*segment).param3,
    //    &_1,
    //    &_2,
    //    &_3
    //);
    //if (_1) {
    //    solution_count = solution_count + 1u;
    //}
    //if (_2) {
    //    solution_count = solution_count + 1u;
    //}
    //if (_3) {
    //    solution_count = solution_count + 1u;
    //}


    var y_inv: bool = false;
    var x_inv: bool = false;
    // left_bot -> right_top
    //if ((*segment).param0.x <= (*segment).param3.x && (*segment).param0.y >= (*segment).param3.y) {
    //}
    // left_top -> right_bot
    if ((*segment).param0.x <= (*segment).param3.x && (*segment).param0.y <= (*segment).param3.y) {
        x_inv = true;
    }
    // right_bot -> left_top
    else if ((*segment).param0.x >= (*segment).param3.x && (*segment).param0.y >= (*segment).param3.y) {
        y_inv = true;
    }
    // right_top -> left_bot
    else if ((*segment).param0.x >= (*segment).param3.x && (*segment).param0.y <= (*segment).param3.y) {
        y_inv = true;
        x_inv = true;
    }

    if (solution_count == 3u) {
        if (y_inv) {
            *x = ev_some(solution.x, true);
            *y = ev_some(solution.y, false);
            *z = ev_some(solution.z, true);
            return;
            //return (pos.y >= solution.y && pos.y <= solution.z) || (pos.y <= solution.x);
        }
        *x = ev_some(solution.x, false);
        *y = ev_some(solution.y, true);
        *z = ev_some(solution.z, false);
        return;
        //return (pos.y >= solution.x && pos.y <= solution.y) || (pos.y >= solution.z);
    }
    else if (solution_count == 2u) {
        if (x_inv) {
            if (pos.x > max((*segment).param0.x, (*segment).param3.x)) {
                *x = ev_some(solution.x, false);
                *y = ev_some(solution.y, true);
                *z = ev_none;
                return;
                //return (pos.y <= solution.x || pos.y >= solution.y);
            }
            *x = ev_some(solution.x, true);
            *y = ev_some(solution.y, false);
            *z = ev_none;
            return;
            //return (pos.y >= solution.x && pos.y <= solution.y);
        }
        if (pos.x < min((*segment).param0.x, (*segment).param3.x)) {
            *x = ev_some(solution.x, false);
            *y = ev_some(solution.y, true);
            *z = ev_none;
            return;
            //return (pos.y <= solution.x || pos.y >= solution.y);
        }
        *x = ev_some(solution.x, true);
        *y = ev_some(solution.y, false);
        *z = ev_none;
        return;
        //return (pos.y >= solution.x && pos.y <= solution.y);
    }
    else if (solution_count == 1u) {
        if (y_inv) {
            *x = ev_some(solution.x, true);
            *y = ev_none;
            *z = ev_none;
            return;
            //return pos.y >= solution.x;
        }
        *x = ev_some(solution.x, false);
        *y = ev_none;
        *z = ev_none;
        return;
        //return pos.y <= solution.x;
    }
    *x = ev_none;
    *y = ev_none;
    *z = ev_none;
    return;
}

// returns in most cases one edge value which corresponds to the intersection point
// Only if the line has a zero greadient then two edge values are returned
fn edge_values_linear(
    pos: vec2<f32>,
    segment: ptr<function, PathSegment>,
    x_out: ptr<function, EdgeValue>,
    y_out: ptr<function, EdgeValue>
) {
    if (pos.x < min((*segment).param0.x, (*segment).param1.x) ||
        pos.x > max((*segment).param0.x, (*segment).param1.x)) {
        *x_out = ev_none;
        *y_out = ev_none;
        return;
    }
    var dx: f32;
    var dy: f32;
    if ((*segment).param0.x < (*segment).param1.x) {
        dx = (*segment).param1.x - (*segment).param0.x;
        dy = (*segment).param1.y - (*segment).param0.y;
    } else {
        dx = (*segment).param0.x - (*segment).param1.x;
        dy = (*segment).param0.y - (*segment).param1.y;
    }
    // If we don't have a gradient
    if (dx == 0.0) {
        *x_out = ev_none;//ev_some(min((*segment).param0.y, (*segment).param1.y), false);
        *y_out = ev_none;//ev_some(max((*segment).param0.y, (*segment).param1.y), true);
        return;
    }
    if (dy == 0.0) {
        if ((*segment).param0.x < (*segment).param1.x) {
            *x_out = ev_some((*segment).param0.y, false);
        } else {
            *x_out = ev_some((*segment).param0.y, true);
        }
        *y_out = ev_none;
        return;
    }
    var top_is_area: bool;
    let y: f32 = (dy / dx) * (pos.x - (*segment).param0.x) + (*segment).param0.y;

    // left_bot -> right_top
    if ((*segment).param0.x < (*segment).param1.x && (*segment).param0.y > (*segment).param1.y) {
        top_is_area = false;
    }
    // left_top -> right_bot
    else if ((*segment).param0.x < (*segment).param1.x && (*segment).param0.y < (*segment).param1.y) {
        top_is_area = false;
    }
    // right_bot -> left_top
    else if ((*segment).param0.x > (*segment).param1.x && (*segment).param0.y > (*segment).param1.y) {
        top_is_area = true;
    }
    // right_top -> left_bot
    else if ((*segment).param0.x > (*segment).param1.x && (*segment).param0.y < (*segment).param1.y) {
        top_is_area = true;
    }
    *x_out = ev_some(y, top_is_area);
    *y_out = ev_none;
    return;
}

[[stage(fragment)]]
fn fs_main(vertex_out: VertexOutput) -> [[location(0)]] vec4<f32> {

    if (false) {

        // Get the width and height of the screen
        let extent: vec2<u32> = unpack(globals.width_height);

        let index_x: u32 = u32(round(vertex_out.norm_position.x * f32(extent.x)));
        let index_y: u32 = u32(round(vertex_out.norm_position.y * f32(extent.y)));
        let solution: vec4<f32> = textureLoad(compute_texture, vec2<i32>(i32(index_x), i32(index_y)), 0);
        //var solution_count: u32 = u32(round(so
        //return vec4<f32>(f32(index) / f32(absolute_segment_width));
        //return vec4<f32>(f32(index) / f32(extent.x), f32(i) / f32(extent.y), 0.0, 0.0);
        //return solution;
        return solution;
    }

    //let extent: vec2<u32> = unpack(globals.width_height);
    // Relative width and height of one pixel
    //let pt: vec2<f32> = vec2<f32>(0.98 / f32(extent.x), 0.98 / f32(extent.y));
    var y_closest_top: EdgeValue = ev_none;
    var y_closest_bot: EdgeValue = ev_none;


    // Go through all path segments and find the closest edge values
    for (var i: u32 = vertex_out.segment_range.x; i < vertex_out.segment_range.y; i = i + 1u) {

        var segment: PathSegment = all_paths.segments[i];

        // If current x vertical line not intersecting with the segment skip the segment
        if (vertex_out.norm_position.x < segment.rect_lu.x || vertex_out.norm_position.x > segment.rect_rl.x) {
            continue;
        }

        if (segment.typ == SEGMENT_TYPE_LINEAR) {
            var x: EdgeValue = ev_none;
            var y: EdgeValue = ev_none;
            edge_values_linear(vertex_out.norm_position, &segment, &x, &y);
            ev_check_closer_top(&y_closest_top, &x, vertex_out.norm_position);
            ev_check_closer_top(&y_closest_top, &y, vertex_out.norm_position);
            ev_check_closer_bot(&y_closest_bot, &x, vertex_out.norm_position);
            ev_check_closer_bot(&y_closest_bot, &y, vertex_out.norm_position);
        } else if (segment.typ == SEGMENT_TYPE_ARC) {

        } else if (segment.typ == SEGMENT_TYPE_QUADRATIC_BEZIER) {

        } else if (segment.typ == SEGMENT_TYPE_CUBIC_BEZIER) {
            if (false) {

                // Get the width and height of the screen
                let extent: vec2<u32> = unpack(globals.width_height);
                let segment_width: f32 = rect_width(segment.rect_lu, segment.rect_rl);
                let absolute_segment_width: u32 = u32(round(segment_width * f32(extent.x)));
                let index: u32 = u32(vertex_out.norm_position.x * f32(absolute_segment_width));
                let solution: vec4<f32> = textureLoad(compute_texture, vec2<i32>(i32(index), i32(i)), 0);
                //var solution_count: u32 = u32(round(solution.w));

                //return vec4<f32>(f32(index) / f32(absolute_segment_width));
                //return vec4<f32>(f32(index) / f32(extent.x), f32(i) / f32(extent.y), 0.0, 0.0);
                //return solution;

            }

            var x: EdgeValue = ev_none;
            var y: EdgeValue = ev_none;
            var z: EdgeValue = ev_none;
            edge_values_cubic_bezier(vertex_out.norm_position, i, &segment, &x, &y, &z);
            ev_check_closer_top(&y_closest_top, &x, vertex_out.norm_position);
            ev_check_closer_top(&y_closest_top, &y, vertex_out.norm_position);
            ev_check_closer_top(&y_closest_top, &z, vertex_out.norm_position);
            ev_check_closer_bot(&y_closest_bot, &x, vertex_out.norm_position);
            ev_check_closer_bot(&y_closest_bot, &y, vertex_out.norm_position);
            ev_check_closer_bot(&y_closest_bot, &z, vertex_out.norm_position);
        } else {

        }
    }


    if (y_closest_top.top_is_area) {
        //return vec4<f32>(y_closest_top.y);
    }

    // If the current point lays in between the two closest edge values at the current x, y coord,
    // Then the point is part of an area
    if (y_closest_top.is_set && y_closest_bot.is_set && !y_closest_top.top_is_area && y_closest_bot.top_is_area) {
        return vertex_out.color;
    }

    return vec4<f32>(0.2);
}

[[stage(vertex)]]
fn vs_main(model: VertexInput, instance: Instance) -> VertexOutput {
    var vertex_out: VertexOutput;
    if (model.vid == 0u || model.vid == 3u) {
        vertex_out.position = cc(vec4<f32>(instance.rect.x, instance.rect.y * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.xy);
    } else if (model.vid == 2u || model.vid == 4u) {
        vertex_out.position = cc(vec4<f32>(instance.rect.z, instance.rect.w * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.z, instance.rect.w);
    } else if (model.vid == 1u) {
        vertex_out.position = cc(vec4<f32>(instance.rect.x, instance.rect.w * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.x, instance.rect.w);
    } else {
        vertex_out.position = cc(vec4<f32>(instance.rect.z, instance.rect.y * globals.aspect_ratio, 0.0, 1.0));
        vertex_out.norm_position = vec2<f32>(instance.rect.z, instance.rect.y);
    }
    vertex_out.color = instance.color;
    vertex_out.segment_range = instance.segment_range;
    return vertex_out;
}