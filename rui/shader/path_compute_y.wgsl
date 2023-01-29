let PI: f32 = 3.14159265358979323846264338327950288;
let SEGMENT_TYPE_LINEAR: u32 = 0u;
let SEGMENT_TYPE_ARC: u32 = 1u;
let SEGMENT_TYPE_QUADRATIC_BEZIER: u32 = 2u;
let SEGMENT_TYPE_CUBIC_BEZIER: u32 = 3u;

let NO_SOLUTION: f32 = -1.0;
let GRADIENT_DIRECTION_X_Y: f32 = 0.0;
let GRADIENT_DIRECTION_X_INV_Y: f32 = 1.0;
let GRADIENT_DIRECTION_X_Y_INV: f32 = 2.0;
let GRADIENT_DIRECTION_X_INV_Y_INV: f32 = 3.0;

fn unpack(x: u32) -> vec2<u32> {
    return vec2<u32>(x >> 16u, x & 0xFFFFu);
}

fn cbrt(x: f32) -> f32 {
    return sign(x) * pow(abs(x), 1./3.);
}

fn quadratic(x: f32, a: f32, b: f32, c: f32) -> f32 {
    return a * x * x + b * x + c;
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

fn cubic_bezier_gradient(
    t: f32,
    a: f32,
    b: f32,
    c: f32
) -> f32 {
    let _2: f32 = 2.0;
    let _3: f32 = 3.0;

    return quadratic(t, _3 * a, _2 * b, c);
}

// Computes the y values of a given x value and the gradient for every y
fn solve_cubic_bezier_y_and_gradients_y(
    x: f32,
    p0: vec2<f32>,
    p1: vec2<f32>,
    p2: vec2<f32>,
    p3: vec2<f32>,
    _0_sol: ptr<function, bool>,
    _1_sol: ptr<function, bool>,
    _2_sol: ptr<function, bool>
) -> mat2x3<f32> {

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
    // two columns 3 rows
    var res: mat2x3<f32> = mat2x3<f32>(vec3<f32>(0.0), vec3<f32>(0.0));
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
        res[0].x = cubic(t.x, ay, by, cy, dy);
        res[1].x = cubic_bezier_gradient(t.x, ay, by, cy);
    }
    if (*_1_sol) {
        res[0].y = cubic(t.y, ay, by, cy, dy);
        res[1].y = cubic_bezier_gradient(t.y, ay, by, cy);
        if (res[0].x > res[0].y) {
            tmp = res[0].x;
            res[0].x = res[0].y;
            res[0].y = tmp;
            tmp = res[1].x;
            res[1].x = res[1].y;
            res[1].y = tmp;
        }
    }
    if (*_2_sol) {
        res[0].z = cubic(t.z, ay, by, cy, dy);
        res[1].z = cubic_bezier_gradient(t.z, ay, by, cy);
        if (res[0].x > res[0].z) {
            tmp = res[0].x;
            res[0].x = res[0].z;
            res[0].z = res[0].y;
            res[0].y = tmp;
            tmp = res[1].x;
            res[1].x = res[1].z;
            res[1].z = res[1].y;
            res[1].y = tmp;
        } else if (res[0].y > res[0].z) {
            tmp = res[0].y;
            res[0].y = res[0].z;
            res[0].z = tmp;
            tmp = res[1].y;
            res[1].y = res[1].z;
            res[1].z = tmp;
        }
    }
    return res;
}

fn rect_height(rect_lu: vec2<f32>, rect_rl: vec2<f32>) -> f32 {
    return rect_rl.y - rect_lu.y;
}

/*
fn direction(start: vec2<f32>, end: vec2<f32>) -> f32 {
    if (start.x <= end.x && start.y > end.y) {
        return GRADIENT_DIRECTION_X_Y;
    } else if (start.x <= end.x && start.y <= end.y) {
        return GRADIENT_DIRECTION_X_Y_INV;
    } else if (start.x > end.x && start.y > end.y) {
        return GRADIENT_DIRECTION_X_INV_Y;
    } else {
        return GRADIENT_DIRECTION_X_INV_Y_INV;
    }
}
*/

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
@group(2) @binding(1) var output_texture_gradient: texture_storage_2d<rgba32float, write>;

let WORKGROUP_SIZE: u32 = 256u;

// We dispatch (height, 1, 1) workgroups
// Each workgroup has a local amount of threads of 128 and is responsible for evaluating one path segment
@compute @workgroup_size(256u)
fn cs_main(@builtin(num_workgroups) num_wg: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_id) liid: vec3<u32>) {

    // Get the width and height of the
    let extent: vec2<u32> = unpack(globals.width_height);
    // Because each workgroup represents a path segment
    // the workgroup_id.x is the width index and therefore the segment index
    let segment_index: u32 = wid.x;
    let segment: PathSegment = all_paths.segments[segment_index];
    let segment_height: f32 = rect_height(segment.rect_lu, segment.rect_rl);
    // The segment width is always normalized, but because path can be scaled down we have to calculate the absolute
    // width by multiplying with the screen width
    //let absolute_segment_width: u32 = u32(round(segment_width * f32(extent.x)));
    // Local thread id marks start pixel position
    // Every round jump amount of workgroup_size to get the next pixel to process
    for (var index: u32 = liid.x; index < extent.x; index = index + WORKGROUP_SIZE) {
        // Calculate the current x position
        let x: f32 = f32(index) / f32(extent.x) + segment.rect_lu.x;
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
            let inv: f32 = sign(segment.param3.y - segment.param0.y);
            var _1: bool = false;
            var _2: bool = false;
            var _3: bool = false;
            var solution: mat2x3<f32> = solve_cubic_bezier_y_and_gradients_y(
                x,
                segment.param0,
                segment.param1,
                segment.param2,
                segment.param3,
                &_1,
                &_2,
                &_3
            );
            if (!_1) {
                solution[0].x = NO_SOLUTION;
            }
            if (!_2) {
                solution[0].y = NO_SOLUTION;
            }
            if (!_3) {
                solution[0].z = NO_SOLUTION;
            }
            // Store solution in the output texture
            textureStore(output_texture, vec2<i32>(i32(index), i32(segment_index)), vec4<f32>(solution[0], x));
            // Store gradient solution in the output gradient texture
            textureStore(output_texture_gradient, vec2<i32>(i32(index), i32(segment_index)), vec4<f32>(solution[1], inv));
        }
    }

}