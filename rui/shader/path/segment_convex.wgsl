struct Globals {
    width_height: u32,
    aspect_ratio: f32,
}

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) segment_index: u32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) segment_index: u32,
    @location(1) uv: vec2<f32>,
}

struct PathSegment {
    param0: vec2<f32>,
    param1: vec2<f32>,
    param2: vec2<f32>,
    param3: vec2<f32>
}

let SEGMENT_TYPE_LINEAR: u32 = 0u;
let SEGMENT_TYPE_ARC: u32 = 1u;
let SEGMENT_TYPE_QUADRATIC_BEZIER: u32 = 2u;
let SEGMENT_TYPE_CUBIC_BEZIER: u32 = 3u;

struct Paths {
    segments: array<PathSegment>
};

fn unpack(x: u32) -> vec2<u32> {
    return vec2<u32>(x >> 16u, x & 0xFFFFu);
}

@group(1) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(0) var<storage, read> all_paths: array<PathSegment>;


fn solve_quadratic(a: f32, b: f32, c: f32, r: ptr<function, vec2<f32>>) -> u32 {
    let disc: f32 = b * b - 4. * a * c;
    if (disc < 0.) {
        // We don't care about complex solutions
        return 0u;
    }
    let t0: f32 = sqrt(disc);
    let t1: f32 = 2. * a;
    let x0: f32 = (-b + t0) / t1;
    let x1: f32 = (-b - t0) / t1;
    (*r).x = x0;
    (*r).y = x1;
    return 2u;
}

/// Exact distance to a line
fn line_dis(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>) -> f32 {
    let dir: vec2<f32> = normalize(p1-p0);

    let h: f32 = min(1.0, max(0.0, dot(dir, uv - p0) / length(p0-p1)));
    let d: f32 = length((uv-p0) - h * (p1 - p0));
    return d;
}

fn line_sign(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>) -> f32 {
    return sign((p1.x - p0.x) * (uv.y - p0.y) - (p1.y - p0.y) * (uv.x - p0.x));
}

/// Cubic bezier

fn parametric_cub_bezier(t: f32, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> vec2<f32> {
	let a0: vec2<f32> = (-p0 + 3. * p1 - 3. * p2 + p3);
    let a1: vec2<f32> = (3. * p0 - 6. * p1 + 3. * p2);
    let a2: vec2<f32> = (-3. * p0 + 3. * p1);
    let a3: vec2<f32> = p0;

    return (((a0 * t) + a1) * t + a2) * t + a3;
}

fn parametric_cub_bezier_der(t: f32, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> vec2<f32> {
	let a0: vec2<f32> = 3. * (-p0 + 3. * p1 - 3. * p2 + p3);
	let a1: vec2<f32> = 2. * (3. * p0 - 6. * p1 + 3. * p2);
	let a2: vec2<f32> = (-3. * p0 + 3. * p1);

	return (a0 * t + a1) * t + a2;
}

//From Trisomie21
//But instead of his cancellation fix i'm using a newton iteration
fn solve_cubic(a: f32, b: f32, c: f32, r: ptr<function, vec3<f32>>) -> u32 {
	let p: f32 = b - a * a / 3.0;
	let q: f32 = a * (2.0 * a * a - 9.0 * b) / 27.0 + c;
	let p3: f32 = p * p * p;
	let d: f32 = q * q + 4.0 * p3 / 27.0;
	let offset: f32 = -a / 3.0;
	if (d >= 0.0) { // Single solution
		let z: f32 = sqrt(d);
		var u: f32 = (-q + z) / 2.0;
		var v: f32 = (-q - z) / 2.0;
		u = sign(u) * pow(abs(u), 1.0 / 3.0);
		v = sign(v) * pow(abs(v), 1.0 / 3.0);
		(*r).x = offset + u + v;


		//Single newton iteration to account for cancellation
		let f: f32 = (((*r).x + a) * (*r).x + b) * (*r).x + c;
		let f1: f32 = (3. * (*r).x + 2. * a) * (*r).x + b;

		(*r).x -= f / f1;

		return 1u;
	}
	let u: f32 = sqrt(-p / 3.0);
	let v: f32 = acos(-sqrt( -27.0 / p3) * q / 2.0) / 3.0;
	let m: f32 = cos(v);
	let n: f32 = sin(v) * 1.732050808;

	//Single newton iteration to account for cancellation
	//(once for every root)

	(*r).x = offset + u * (m + m);
    (*r).y = offset - u * (n + m);
    (*r).z = offset + u * (n - m);

	let f: vec3<f32> = (((*r) + a) * (*r) + b) * (*r) + c;
	let f1: vec3<f32> = (3. * (*r) + 2. * a) * (*r) + b;

	(*r) -= f / f1;

	return 3u;
}

//Sign computation is pretty straightforward:
//I'm solving a cubic equation to get the intersection count
//of a ray from the current point to infinity and parallel to the x axis
//Also i'm computing the intersection count with the tangent in the end points of the curve
fn cubic_bezier_sign(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> f32 {

	let cu: f32 = (-p0.y + 3. * p1.y - 3. * p2.y + p3.y);
	let qu: f32 = (3. * p0.y - 6. * p1.y + 3. * p2.y);
	let li: f32 = (-3. * p0.y + 3. * p1.y);
	let co: f32 = p0.y - uv.y;

	var roots: vec3<f32>;
	let n_roots: u32 = solve_cubic(qu/cu, li/cu, co / cu, &roots);

	var n_ints: u32 = 0u;

	for (var i: u32 = 0u; i < 3u; i = i + 1u) {
		if (i < n_roots) {
			if (roots[i] >= 0. && roots[i] <= 1.) {
				let x_pos: f32 = (
				    (((-p0.x + 3. * p1.x - 3. * p2.x + p3.x) * roots[i] + (3. * p0.x - 6. * p1.x + 3. * p2.x)) * roots[i]) +
				    (-3. * p0.x + 3. * p1.x)
				) * roots[i] + p0.x;
				if (x_pos < uv.x) {
					n_ints = n_ints + 1u;
				}
			}
		}
	}


	let tang1: vec2<f32> = p0.xy - p1.xy;
	let tang2: vec2<f32> = p2.xy - p3.xy;

	let nor1: vec2<f32> = vec2<f32>(tang1.y, -tang1.x);
	let nor2: vec2<f32> = vec2<f32>(tang2.y, -tang2.x);

	if (p0.y < p1.y){
		if ((uv.y <= p0.y) && (dot(uv - p0.xy, nor1) < 0.)) {
			n_ints = n_ints + 1u;
		}
	}
	else {
		if (!(uv.y <= p0.y) && !(dot(uv - p0.xy, nor1) < 0.)) {
			n_ints = n_ints + 1u;
		}
	}

	if (p2.y < p3.y){
		if (!(uv.y <= p3.y) && dot(uv - p3.xy, nor2) < 0.) {
			n_ints = n_ints + 1u;
		}
	}
	else {
		if ((uv.y <= p3.y) && !(dot(uv - p3.xy, nor2) < 0.)) {
			n_ints = n_ints + 1u;
		}
	}

	if (n_ints == 0u || n_ints == 2u || n_ints == 4u) {
		return 1.;
	}
	else {
		return -1.;
	}
}

/// Own sign function see https://matheplanet.com/matheplanet/nuke/html/viewtopic.php?topic=261421
fn cubic_bezier_sign2(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t_min: f32) -> f32 {
    let to_curve: vec2<f32> = uv - parametric_cub_bezier(t_min, p0, p1, p2, p3);
    let tang: vec2<f32> = parametric_cub_bezier_der(t_min, p0, p1, p2, p3);
    var smat: mat2x2<f32> = mat2x2<f32>(to_curve, tang);
    return sign(determinant(smat));
}

let num_iterations: u32 = 3u;
let num_start_params: u32 = 3u;
let factor: f32 = 1.;

fn cubic_bezier_normal_iteration2(t: f32, a0: vec2<f32>, a1: vec2<f32>, a2: vec2<f32>, a3: vec2<f32>) -> f32 {
	//horner's method
	let a_2: vec2<f32> = a2 + t * a3;
	let a_1: vec2<f32> = a1 + t * a_2;
	let b_2: vec2<f32> = a_2 + t * a3;

	let uv_to_p: vec2<f32> = a0 + t * a_1;
	let tang: vec2<f32> = a_1 + t * b_2;
	let snd_drv: vec2<f32> = 2. * (b_2 + t * a3);

	let l_tang: f32 = dot(tang, tang);

	let fac: f32 = dot(tang, snd_drv) / (2. * l_tang);
	let d: f32 = -dot(tang, uv_to_p);

	let t2: f32 = d / (l_tang + fac * d);

	return t + factor * t2;
}

fn cubic_bezier_dis_approx_sq(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t_min: ptr<function, f32>) -> f32 {
	let a3: vec2<f32> = (-p0 + 3. * p1 - 3. * p2 + p3);
	let a2: vec2<f32> = (3. * p0 - 6. * p1 + 3. * p2);
	let a1: vec2<f32> = (-3. * p0 + 3. * p1);
	let a0: vec2<f32> = p0 - uv;

	var d0: f32 = 1e38;

	var t0: f32 = 0.;
	var t: f32;

	for(var i: u32 = 0u; i < num_start_params; i = i + 1u) {
		t = t0;
		for(var j: u32 = 0u; j < num_iterations; j = j + 1u) {
			t = cubic_bezier_normal_iteration2(t, a0, a1, a2, a3);
		}
		t = clamp(t, 0., 1.);
		let uv_to_p: vec2<f32> = ((a3 * t + a2) * t + a1) * t + a0;
		d0 = min(d0, dot(uv_to_p, uv_to_p));

		t0 = t0 + 1. / f32(num_start_params - 1u);
	}
    *t_min = t;
	return d0;
}

fn cubic_bezier_dis_approx(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t_min: ptr<function, f32>) -> f32 {
	return sqrt(cubic_bezier_dis_approx_sq(uv, p0, p1, p2, p3, t_min));
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    let pos = vec2(model.pos.x / globals.aspect_ratio, model.pos.y);

    var out: VertexOutput;
    out.pos = vec4<f32>(pos, 0.0, 1.0);
    out.uv = model.pos;
    out.segment_index = model.segment_index;
    return out;
}

@fragment
fn fs_main(model: VertexOutput) -> @location(0) vec4<f32> {

    let extent: vec2<u32> = unpack(globals.width_height);
    let max_pixel_size: f32 = max(2.0f / f32(extent.x), 2.0f / f32(extent.y));
    let segment: PathSegment = all_paths[model.segment_index];
    let line_sgn = line_sign(model.uv, segment.param0, segment.param3);
    let line_dis = line_dis(model.uv, segment.param0, segment.param3);

    var t_min: f32;
    let dis = cubic_bezier_dis_approx(model.uv, segment.param0, segment.param1, segment.param2, segment.param3, &t_min);
    let sgn = cubic_bezier_sign2(model.uv, segment.param0, segment.param1, segment.param2, segment.param3, t_min);
    let s_dist = dis * sgn;

    let px = dpdx(model.uv);
    let py = dpdy(model.uv);

    let fx = (2.0*model.uv.x)*px.x - px.y;
    let fy = (2.0*model.uv.x)*py.x - py.y;

    let sd = (model.uv.x*model.uv.x - model.uv.y) / sqrt(fx*fx + fy*fy);
    let alpha = 0.5 - sd;
    /*
    if (dis <= max_pixel_size * 2.) {
        let _Cutoff = dis / max_pixel_size * 2.;

    }*/

    // Fragment is on the line
    if (line_dis <= max_pixel_size * 5. && line_sgn > 0.) {
        //return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        //discard;
    }

    /// This is the concave region
    if (line_sgn >= 0. && sgn >= 0.) {
        //return vec4<f32>(1.0, 0.0, 0.0, alpha);
    }
    /// This is the convex region
    if (line_sgn <= 0. && sgn < 0.) {
        return vec4(1.0);
    }
    if (line_sgn <= 0. && sgn > 0. && dis < max_pixel_size) {
        return vec4(1. - smoothstep(0., max_pixel_size, dis));
    }

    if (line_dis <= max_pixel_size * 1.5 && sgn < 0.) {
        return vec4(smoothstep(0., max_pixel_size, dis));
    }

    if (dis < max_pixel_size && line_sgn <= 0.) {
        //return vec4<f32>(0.0, 1.0, 0.0, 1. - alpha);
    }

    discard;
/*
    let col1: f32 = distance(model.uv, segment.param0);
    let col2: f32 = distance(model.uv, segment.param3);

    return vec4<f32>(0.0, 0.0, 1.0, alpha);*/
}