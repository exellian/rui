let PI: f32 = 3.14159265358979323846264338327950288;
let SEGMENT_TYPE_LINEAR: u32 = 0u;
let SEGMENT_TYPE_ARC: u32 = 1u;
let SEGMENT_TYPE_QUADRATIC_BEZIER: u32 = 2u;
let SEGMENT_TYPE_CUBIC_BEZIER: u32 = 3u;

let NO_SOLUTION: f32 = -1.0;

fn unpack(x: u32) -> vec2<u32> {
    return vec2<u32>(x >> 16u, x & 0xFFFFu);
}

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

// Computes the x values of a given y value
fn solve_cubic_bezier_x(
    y: f32,
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
    let dx: f32 = p0.x;

    let ay: f32 = -p0.y + _3 * p1.y - _3 * p2.y + p3.y;
    let by: f32 = _3 * p0.y - _6 * p1.y + _3 * p2.y;
    let cy: f32 = -_3 * p0.y + _3 * p1.y;
    let dy: f32 = p0.y - y;

    // Get the times of the curve where b(t) = x
    var t: vec3<f32> = solve_cubic(ay, by, cy, dy, _0_sol, _1_sol, _2_sol);
    // two columns 3 rows
    var res: vec3<f32> = vec3<f32>(0.0);
    var tmp: f32 = 0.0;

    // If the intersection point is not in between start and end range
    // throw it away
    if (*_0_sol && (t.x < -0.0 || t.x > 1.0)) {
        *_0_sol = false;
        t.x = 0.0;
    }
    if (*_1_sol && (t.y < -0.0 || t.y > 1.0)) {
        *_1_sol = false;
        t.y = 0.0;
    }
    if (*_2_sol && (t.z < -0.0 || t.z > 1.0)) {
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
        res.x = cubic(t.x, ax, bx, cx, dx);
    }
    if (*_1_sol) {
        res.y = cubic(t.y, ax, bx, cx, dx);
        if (res.x > res.y) {
            tmp = res.x;
            res.x = res.y;
            res.y = tmp;
        }
    }
    if (*_2_sol) {
        res.z = cubic(t.z, ax, bx, cx, dx);
        if (res.x > res.z) {
            tmp = res.x;
            res.x = res.z;
            res.z = res.y;
            res.y = tmp;
        } else if (res.y > res.z) {
            tmp = res.y;
            res.y = res.z;
            res.z = tmp;
        }
    }
    return res;
}

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
    param3: vec2<f32>,
}

struct Paths {
    segments: array<PathSegment>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<storage,read> all_paths: Paths;
@group(2) @binding(0) var output_texture: texture_storage_2d<rgba32float, write>;
@group(2) @binding(1) var output_texture_area_directions: texture_storage_2d<rgba32uint, write>;

let WORKGROUP_SIZE: u32 = 256u;


fn find_solutions_area_direction_x(
    y: f32,
    segment: ptr<function, PathSegment>,
    solutions: vec3<f32>,
    solution_count: u32
) -> vec3<u32> {

    var y_inv: bool = false;
    var x_inv: bool = false;

    // left_top -> right_bot
    if ((*segment).param0.y <= (*segment).param3.y && (*segment).param0.x <= (*segment).param3.x) {
        x_inv = true;
    }
    // right_bot -> left_top
    else if ((*segment).param0.y >= (*segment).param3.y && (*segment).param0.x >= (*segment).param3.x) {
        y_inv = true;
    }
    // right_top -> left_bot
    else if ((*segment).param0.y >= (*segment).param3.y && (*segment).param0.x <= (*segment).param3.x) {
        y_inv = true;
        x_inv = true;
    }

    if (solution_count == 3u) {
        if (y_inv) {
            return vec3<u32>(1u, 0u, 1u);
        }
        return vec3<u32>(0u, 1u, 0u);
    }
    else if (solution_count == 2u) {
        if (x_inv) {
            if (y > max((*segment).param0.y, (*segment).param3.y)) {
                return vec3<u32>(0u, 1u, 0u);
            }
            return vec3<u32>(1u, 0u, 0u);
        }
        if (y < min((*segment).param0.y, (*segment).param3.y)) {
            return vec3<u32>(0u, 1u, 0u);
        }
        return vec3<u32>(1u, 0u, 0u);
    }
    else { //if (solution_count == 1u) {
        if (y_inv) {
            return vec3<u32>(1u, 0u, 0u);
        }
        return vec3<u32>(0u, 0u, 0u);
    }
}

// We dispatch (height, 1, 1) workgroups
// Each workgroup has a local amount of threads of 128 and is responsible for evaluating one path segment
@compute @workgroup_size(256u)
fn cs_main(@builtin(num_workgroups) num_wg: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_id) liid: vec3<u32>) {

    // Get the width and height of the
    let extent: vec2<u32> = unpack(globals.width_height);
    // Because each workgroup represents a path segment
    // the workgroup_id.x is the width index and therefore the segment index
    let segment_index: u32 = wid.x;
    var segment: PathSegment = all_paths.segments[segment_index];
    // The segment width is always normalized, but because path can be scaled down we have to calculate the absolute
    // width by multiplying with the screen width
    //let absolute_segment_width: u32 = u32(round(segment_width * f32(extent.x)));
    // Local thread id marks start pixel position
    // Every round jump amount of workgroup_size to get the next pixel to process
    for (var index: u32 = liid.x; index < extent.y; index = index + WORKGROUP_SIZE) {
        // Calculate the current x position
        let y: f32 = f32(index) / f32(extent.y) + segment.rect_lu.y;
        // We do not solve anything for linear paths in the prepass
        if (segment.typ == SEGMENT_TYPE_LINEAR) {
            return;
        } else if (segment.typ == SEGMENT_TYPE_ARC) {
            // TODO
            return;
        } else if (segment.typ == SEGMENT_TYPE_QUADRATIC_BEZIER) {
            // TODO
            return;
        } else if (segment.typ == SEGMENT_TYPE_CUBIC_BEZIER) {
            var _1: bool = false;
            var _2: bool = false;
            var _3: bool = false;
            var solution_count: u32 = 0u;
            var solutions: vec3<f32> = solve_cubic_bezier_x(
                y,
                segment.param0,
                segment.param1,
                segment.param2,
                segment.param3,
                &_1,
                &_2,
                &_3
            );

            if (!_1) {
                solutions.x = NO_SOLUTION;
                solution_count = solution_count + 1u;
            }
            if (!_2) {
                solutions.y = NO_SOLUTION;
                solution_count = solution_count + 1u;
            }
            if (!_3) {
                solutions.z = NO_SOLUTION;
                solution_count = solution_count + 1u;
            }
            let area_directions: vec3<u32> = find_solutions_area_direction_x(y, &segment, solutions, solution_count);

            // Store solution in the output texture
            textureStore(output_texture, vec2<i32>(i32(index), i32(segment_index)), vec4<f32>(solutions, y));

            // Store gradient solution in the output gradient texture
            textureStore(output_texture_area_directions, vec2<i32>(i32(index), i32(segment_index)), vec4<u32>(area_directions, 0u));
        }
    }

}