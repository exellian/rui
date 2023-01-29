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
    x: f32;
    grad: f32;
    direction: f32;
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

let NO_SOLUTION: f32 = -1.0;
let GRADIENT_DIRECTION_X_Y: f32 = 0.0;
let GRADIENT_DIRECTION_X_INV_Y: f32 = 1.0;
let GRADIENT_DIRECTION_X_Y_INV: f32 = 2.0;
let GRADIENT_DIRECTION_X_INV_Y_INV: f32 = 3.0;


// Globals
[[group(0), binding(0)]] var<uniform> globals: Globals;
// A storage buffer, for the path segments
[[group(1), binding(0)]] var<storage, read> all_paths: Paths;
// A texture containing path segment solving results that got computed in a
// previous render pass
[[group(2), binding(0)]] var compute_texture: texture_2d<f32>;
[[group(2), binding(1)]] var compute_texture_gradient: texture_2d<f32>;

let ev_none: EdgeValue = EdgeValue(0.0, false, false);

fn ev_some(y: f32, top_is_area: bool, x: f32, grad: f32, direction: f32) -> EdgeValue {
    var val: EdgeValue;
    val.y = y;
    val.top_is_area = top_is_area;
    val.is_set = true;
    val.x = x;
    val.grad = grad;
    val.direction = direction;
    return val;
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

fn valid(x: f32, o: f32, size: f32) -> bool {
    return o < x && x < (o + size);
}

fn linearf(x: f32, m: f32, t: f32) -> f32 {
    return m * x + t;
}

fn linearinvf(y: f32, m: f32, t: f32) -> f32 {
    return (y - t) / m;
}

fn area_opposite(a: f32, b: f32, o: f32, size: f32) -> f32 {
    return size * (a - o + (b - a) / 2.0);
}

fn area_inv(area: f32, size: f32) -> f32 {
    return size * size - area;
}

fn side_length_close(a: f32, o: f32) -> f32 {
    return a - o;
}

fn side_length_far(a: f32, o: f32, size: f32) -> f32 {
    return o + size - a;
}

fn flip_area_if(flip: bool, area: f32, size: f32) -> f32 {
    if (flip) {
        return area_inv(area, size);
    }
    return area;
}

fn area(x: f32, y: f32, m: f32, o: vec2<f32>, size: f32, points_down: bool, points_down_rotated: bool) -> f32 {
    //if x_diff == 0.0:
    //    y_left = -o_y
    //    y_right = -o_y
    //    x_upper = p_max[0]
    //    x_lower = p_max[0]
    //elif y_diff == 0.0:
    //    y_left = p_max[1]
    //    y_right = p_max[1]
    //    x_upper = -o_x
    //    x_lower = -o_x
    //else:
    let t: f32 = y - m * x;
    let y_left: f32 = linearf(o.x, m, t);
    let y_right: f32 = linearf(o.x + size, m, t);
    let x_upper: f32 = linearinvf(o.y, m, t);
    let x_lower: f32 = linearinvf(o.y + size, m, t);

    if (valid(x_upper, o.x, size) && valid(x_lower, o.x, size)) {
        let tmp: f32 = area_opposite(x_upper, x_lower, o.x, size);
        return flip_area_if(!points_down, tmp, size);
    }
    if (valid(y_left, o.y, size) && valid(y_right, o.y, size)) {
        let tmp: f32 = area_opposite(y_left, y_right, o.y, size);
        return flip_area_if(!points_down_rotated, tmp, size);
    }
    if (valid(x_upper, o.x, size) && valid(y_left, o.y, size)) {
        let tmp: f32 = side_length_close(x_upper, o.x) * side_length_close(y_left, o.y);
        return flip_area_if(!points_down, tmp, size);
    }
    if (valid(x_upper, o.x, size) && valid(y_right, o.y, size)) {
        let tmp: f32 = side_length_far(x_upper, o.x, size) * side_length_close(y_right, o.y);
        return flip_area_if(points_down, tmp, size);
    }
    if (valid(x_lower, o.x, size) && valid(y_right, o.y, size)) {
        let tmp: f32 = side_length_far(x_lower, o.x, size) * side_length_far(y_right, o.y, size);
        return flip_area_if(points_down, tmp, size);
    }
    //if (valid(x_lower, o.x, size) && valid(y_left, o.y, size)) {
    let tmp: f32 = side_length_close(x_lower, o.x) * side_length_far(y_left, o.y, size);
    return flip_area_if(!points_down, tmp, size);
    //}

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
    let grads: vec4<f32> = textureLoad(compute_texture_gradientm, vec2<i32>(i32(index), i32(segment_index)), 0);
    var solution_count: u32 = 0u;
    if (solution.x > NO_SOLUTION) {
        solution_count = solution_count + 1u;
    }
    if (solution.y > NO_SOLUTION) {
        solution_count = solution_count + 1u;
    }
    if (solution.z > NO_SOLUTION) {
        solution_count = solution_count + 1u;
    }

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
            *x = ev_some(solution.x, true, solution.w, grads.x, grads.w);
            *y = ev_some(solution.y, false, solution.w, grads.y, grads.w);
            *z = ev_some(solution.z, true, solution.w, grads.z, grads.w);
            return;
            //return (pos.y >= solution.y && pos.y <= solution.z) || (pos.y <= solution.x);
        }
        *x = ev_some(solution.x, false, solution.w, grads.x, grads.w);
        *y = ev_some(solution.y, true, solution.w, grads.y, grads.w);
        *z = ev_some(solution.z, false, solution.w, grads.z, grads.w);
        return;
        //return (pos.y >= solution.x && pos.y <= solution.y) || (pos.y >= solution.z);
    }
    else if (solution_count == 2u) {
        if (x_inv) {
            if (pos.x > max((*segment).param0.x, (*segment).param3.x)) {
                *x = ev_some(solution.x, false, solution.w, grads.x, grads.w);
                *y = ev_some(solution.y, true, solution.w, grads.y, grads.w);
                *z = ev_none;
                return;
                //return (pos.y <= solution.x || pos.y >= solution.y);
            }
            *x = ev_some(solution.x, true, solution.w, grads.x, grads.w);
            *y = ev_some(solution.y, false, solution.w, grads.y, grads.w);
            *z = ev_none;
            return;
            //return (pos.y >= solution.x && pos.y <= solution.y);
        }
        if (pos.x < min((*segment).param0.x, (*segment).param3.x)) {
            *x = ev_some(solution.x, false, solution.w, grads.x, grads.w);
            *y = ev_some(solution.y, true, solution.w, grads.y, grads.w);
            *z = ev_none;
            return;
            //return (pos.y <= solution.x || pos.y >= solution.y);
        }
        *x = ev_some(solution.x, true, solution.w, grads.x, grads.w);
        *y = ev_some(solution.y, false, solution.w, grads.y, grads.w);
        *z = ev_none;
        return;
        //return (pos.y >= solution.x && pos.y <= solution.y);
    }
    else if (solution_count == 1u) {
        if (y_inv) {
            *x = ev_some(solution.x, true, solution.w, grads.x, grads.w);
            *y = ev_none;
            *z = ev_none;
            return;
            //return pos.y >= solution.x;
        }
        *x = ev_some(solution.x, false, solution.w, grads.x, grads.w);
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
            *x_out = ev_some((*segment).param0.y, false, pos.x, 0.0, GRADIENT_DIRECTION_X_Y);
        } else {
            *x_out = ev_some((*segment).param0.y, true, pos.x, 0.0, GRADIENT_DIRECTION_X_Y_INV);
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
    *x_out = ev_some(y, top_is_area, pos.x, (dy / dx), GRADIENT_DIRECTION_X_Y);
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

    let background: vec3<f32> = vec3<f32>(0.2);
    let extent: vec2<u32> = unpack(globals.width_height);
    let pixel_height: f32 = 1.0 / f32(extent.y);

    if (y_closest_top.is_set && !y_closest_top.top_is_area) {
        let d_top: f32 = vertex_out.norm_position.y - y_closest_top.y;
        if (d_top <= pixel_height* 2.0) {
            let fact: f32 = d_top / pixel_height;
            return vec4<f32>(vertex_out.color.xyz * fact + (1.0 - fact) * background, 1.0);
        }
    }

    if (y_closest_top.is_set && y_closest_top.top_is_area) {
        let d_top: f32 = vertex_out.norm_position.y - y_closest_top.y;

        if (d_top <= pixel_height * 1.0) {
            let fact: f32 = d_top / pixel_height;
            return vec4<f32>(vertex_out.color.xyz * (1.0 - fact) +  fact * background, 1.0);
            //return vec4<f32>(0.0, 0.0, 1.0, 1.0);
        }
    }

    // If the current point lays in between the two closest edge values at the current x, y coord,
    // Then the point is part of an area
    if (y_closest_top.is_set && y_closest_bot.is_set && !y_closest_top.top_is_area && y_closest_bot.top_is_area) {
        return vertex_out.color;
    }

    return vec4<f32>(background, 1.0);
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