struct Globals {
    aspect_ratio: f32;
};

struct PathSegment {
    typ: u32;
    woff_param: u32;
    param0: vec2<f32>;
    param1: vec2<f32>;
    param2: vec2<f32>;
    param3: vec2<f32>;
};

struct Paths {
    segments: array<PathSegment, 256>;
};

struct VertexInput {
    [[builtin(vertex_index)]] vid: u32;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0), interpolate(flat)]] segment_range: vec2<u32>;
};

struct Instance {
    [[location(0)]] rect: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] segment_range: vec2<u32>;
};

[[group(1), binding(0)]] var<uniform> all_paths: Paths;

fn cbrt(x: f32) -> f32 {
    return sign(x) * pow(abs(x), 1./3.);
}

fn solve_linear(
    a: f32,
    b: f32,
    _0_sol: ptr<function, bool>
) -> f32 {
    if (a == 0.0) {
        if (b == 0.0) {
            *_0_sol = true;
            return 0.0;
        }
        *_0_sol = false;
        return 0.0;
    }
    *_0_sol = true;
    return -b / a;
}

fn solve_quadratic(
    a: f32,
    b: f32,
    c: f32,
    _0_sol: ptr<function, bool>,
    _1_sol: ptr<function, bool>
) -> vec2<f32> {
    if (a == 0.0) {
        let x: f32 = solve_linear(b, c, _0_sol);
        *_1_sol = false;
        return vec2<f32>(x, 0.0);
    }
    let _2: f32 = 2.0;
    let _4: f32 = 4.0;

    let delta0: f32 = _2 * a;
    let delta1: f32 = -b / delta0;
    let d: f32 = sqrt(b * b - _4 * a * c);
    *_0_sol = true;
    *_1_sol = true;
    return vec2<f32>(delta1 + d / delta0, delta1 - d / delta0);
}

let TWO_THIRD_PI: f32 = 2.09439510239319526263557236234191805;

fn solve_cubic(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    _0_sol: ptr<function, bool>,
    _1_sol: ptr<function, bool>,
    _2_sol: ptr<function, bool>
) -> vec3<f32> {
    if (a == 0.0) {
        let res: vec2<f32> = solve_quadratic(b, c, d, _0_sol, _1_sol);
        *_2_sol = false;
        return vec3<f32>(res.x, res.y, 0.0);
    }

    let a2: f32 = b / a;
    let a1: f32 = c / a;
    let a0: f32 = d / a;

    let _2: f32 = 2.0;
    let _3: f32 = 3.0;
    let _4: f32 = 4.0;
    let _9: f32 = 9.0;
    let _18: f32 = 18.0;
    let _27: f32 = 27.0;
    let _54: f32 = 54.0;

    let q: f32 = (_3 * a1 - a2 * a2) / _9;
    let r: f32 = (_9 * a2 * a1 - _27 * a0 - _2 * a2 * a2 * a2) / _54;
    let q3: f32 = q * q * q;
    let d0: f32 = q3 + r * r;
    let a2_div_3: f32 = a2 / _3;

    if (d0 < 0.0) {
        let phi_3: f32 = acos(r / sqrt(-q3)) / _3;
        let sqrt_q_2: f32 = _2 * sqrt(-q);

        *_0_sol = true;
        *_1_sol = true;
        *_2_sol = true;
        return vec3<f32>(
            sqrt_q_2 * cos(phi_3) - a2_div_3,
            sqrt_q_2 * cos(phi_3 - TWO_THIRD_PI) - a2_div_3,
            sqrt_q_2 * cos(phi_3 + TWO_THIRD_PI) - a2_div_3
        );
    }
    let sqrt_d: f32 = sqrt(d0);
    let s: f32 = cbrt(r + sqrt_d);
    let t: f32 = cbrt(r - sqrt_d);
    let st: f32 = s + t;

    if (s == t && st != 0.0) {
        *_0_sol = true;
        *_1_sol = true;
        *_2_sol = false;
        return vec3<f32>(
            st - a2_div_3,
            -st / _2 - a2_div_3,
            0.0
        );
    }
    *_0_sol = true;
    *_1_sol = false;
    *_2_sol = false;
    return vec3<f32>(
        st - a2_div_3,
        0.0,
        0.0
    );
}

fn cubic(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let x_squared: f32 = x*x;
    return x_squared*x*a + x_squared*b + x*c + d;
}

// Computes the y values of a given x value
fn solve_cubic_bezier(
    x: f32,
    p0: vec2<f32>,
    p1: vec2<f32>,
    p2: vec2<f32>,
    p3: vec2<f32>,
    _0_sol: ptr<function, bool>,
    _1_sol: ptr<function, bool>,
    _2_sol: ptr<function, bool>
) -> vec3<f32> {

    let _3: f32 = 3.0;
    let _6: f32 = 6.0;

    let ax: f32 = -p0.x + _3 * p1.x - _3 * p2.x + p3.x;
    let bx: f32 = _3 * p0.x - _6 * p1.x + _3 * p2.x;
    let cx: f32 = -_3 * p0.x + _3 * p1.x;
    let dx: f32 = p0.x - x;

    let ay: f32 = -p0.y + _3 * p1.y - _3 * p2.y + p3.y;
    let by: f32 = _3 * p0.y - _6 * p1.y + _3 * p2.y;
    let cy: f32 = -_3 * p0.y + _3 * p1.y;
    let dy: f32 = p0.y;

    // Get the times of the curve where b(t) = x
    var t: vec3<f32> = solve_cubic(ax, bx, cx, dx, _0_sol, _1_sol, _2_sol);
    var tmp: f32 = 0.0;

    // If the intersection point is not in between start and end range
    // throw it away
    if (*_0_sol && (t.x < -0.0001 || t.x > 1.0001)) {
        *_0_sol = false;
        t.x = 0.0;
    }
    if (*_1_sol && (t.y < -0.0001 || t.y > 1.0001)) {
        *_1_sol = false;
        t.y = 0.0;
    }
    if (*_2_sol && (t.z < -0.0001 || t.z > 1.0001)) {
        *_2_sol = false;
        t.z = 0.0;
    }
    //pack solutions
    if (!*_0_sol) {
        *_0_sol = *_1_sol;
        *_1_sol = *_2_sol;
        *_2_sol = false;
        t.x = t.y;
        t.y = t.z;
        t.z = 0.0;
    }
    if (!*_1_sol) {
        *_1_sol = *_2_sol;
        *_2_sol = false;
        t.y = t.z;
        t.z = 0.0;
    }

    if (*_0_sol) {
        t.x = cubic(t.x, ay, by, cy, dy);
    }
    if (*_1_sol) {
        t.y = cubic(t.y, ay, by, cy, dy);
        if (t.x > t.y) {
            tmp = t.x;
            t.x = t.y;
            t.y = tmp;
        }
    }
    if (*_2_sol) {
        t.z = cubic(t.z, ay, by, cy, dy);
        if (t.x > t.z) {
            tmp = t.x;
            t.x = t.z;
            t.z = t.y;
            t.y = tmp;
        } else if (t.y > t.z) {
            tmp = t.y;
            t.y = t.z;
            t.z = tmp;
        }
    }
    return t;
}
[[stage(fragment)]]
fn fs_main(vertex_in: VertexOutput) -> [[location(0)]] vec4<f32> {

    //if (true) {
    //    return vec4<f32>(1.0);
    //}

    var y_closest_top: EdgeValue = ev_none;
    var y_closest_bot: EdgeValue = ev_none;

    // Go through all path segments and find the closest edge values
    for (var i: u32 = in.segment_range.x; i < in.segment_range.y; i = i + 1u) {
        var segment: PathSegment = all_paths.segments[i];
        if (segment.typ == SEGMENT_TYPE_LINEAR) {
            var x: EdgeValue = ev_none;
            var y: EdgeValue = ev_none;
            edge_values_linear(in.norm_position, &segment, &x, &y);
            ev_check_closer_top(&y_closest_top, &x, in.norm_position);
            ev_check_closer_top(&y_closest_top, &y, in.norm_position);
            ev_check_closer_bot(&y_closest_bot, &x, in.norm_position);
            ev_check_closer_bot(&y_closest_bot, &y, in.norm_position);
        } else if (segment.typ == SEGMENT_TYPE_ARC) {

        } else if (segment.typ == SEGMENT_TYPE_QUADRATIC_BEZIER) {

        } else if (segment.typ == SEGMENT_TYPE_CUBIC_BEZIER) {
            var x: EdgeValue = ev_none;
            var y: EdgeValue = ev_none;
            var z: EdgeValue = ev_none;
            edge_values_cubic_bezier(in.norm_position, &segment, &x, &y, &z);
            ev_check_closer_top(&y_closest_top, &x, in.norm_position);
            ev_check_closer_top(&y_closest_top, &y, in.norm_position);
            ev_check_closer_top(&y_closest_top, &z, in.norm_position);
            ev_check_closer_bot(&y_closest_bot, &x, in.norm_position);
            ev_check_closer_bot(&y_closest_bot, &y, in.norm_position);
            ev_check_closer_bot(&y_closest_bot, &z, in.norm_position);
        }
    }
    return vec4<f32>(0.2);
}

[[stage(vertex)]]
fn vs_main(model: VertexInput, instance: Instance) -> VertexOutput {
    var vertex_out: VertexOutput;
    if (model.vid == 0u || model.vid == 3u) {
        vertex_out.position = cc(vec4<f32>(0.0, 0.0, 0.0, 1.0));
    } else if (model.vid == 2u || model.vid == 4u) {
        vertex_out.position = cc(vec4<f32>(1.0, 1.0, 0.0, 1.0));
    } else if (model.vid == 1u) {
        vertex_out.position = cc(vec4<f32>(0.0, 1.0, 0.0, 1.0));
    } else {
        vertex_out.position = cc(vec4<f32>(1.0, 0.0, 0.0, 1.0));
    }
    vertex_out.segment_range = instance.segment_range;
    return vertex_out;
}