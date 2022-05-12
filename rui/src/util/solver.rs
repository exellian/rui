use crate::math::{Vec2, Vector};

/*
#[inline]
fn solve_linear(a: f32, b: f32) -> Option<f32> {
    if a == 0.0 {
        if b == 0.0 {
            return Some(0.0);
        }
        return None;
    }
    return Some(-b / a);
}

#[inline]
fn solve_quadratic(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>) {
    if a == 0.0 {
        let x = solve_linear(b, c);
        return (x, None);
    }
    const _0: f32 = 0.0;
    const _2: f32 = 2.0;
    const _4: f32 = 4.0;

    let delta0 = _2 * a;
    let delta1 = -b / delta0;
    let discriminant = b * b - _4 * a * c;
    if discriminant < _0 {
        return (None, None);
    }
    if discriminant == _0 {
        return (Some(delta1), None);
    }
    let d = (b * b - _4 * a * c).sqrt();
    return (Some(delta1 + d / delta0), Some(delta1 - d / delta0));
}

#[inline]
#[allow(dead_code)] // todo
pub fn solve_cubic(a: f32, b: f32, c: f32, d: f32) -> (f32, Option<f32>, Option<f32>) {
    if a == 0. {
        let x = solve_quadratic(b, c, d);
        (x.0, Some(x.1), None)
    } else {
        const TWO_THIRD_PI: f32 = std::f32::consts::PI * 2. / 3.;

        let a2 = b / a;
        let a1 = c / a;
        let a0 = d / a;

        let _2 = f32::from(2i16);
        let _3 = f32::from(3i16);
        let _4 = f32::from(4i16);
        let _9 = f32::from(9i16);
        let _18 = f32::from(18i16);
        let _27 = f32::from(27i16);
        let _54 = f32::from(54i16);

        let q = (_3 * a1 - a2 * a2) / _9;
        let r = (_9 * a2 * a1 - _27 * a0 - _2 * a2 * a2 * a2) / _54;
        let q3 = q * q * q;
        let d = q3 + r * r;
        let a2_div_3 = a2 / _3;

        if d < 0. {
            let phi_3 = (r / (-q3).sqrt()).acos() / _3;
            let sqrt_q_2 = _2 * (-q).sqrt();
            (
                sqrt_q_2 * phi_3.cos() - a2_div_3,
                Some(sqrt_q_2 * (phi_3 - TWO_THIRD_PI).cos() - a2_div_3),
                Some(sqrt_q_2 * (phi_3 + TWO_THIRD_PI).cos() - a2_div_3),
            )
        } else {
            let sqrt_d = d.sqrt();
            let s = (r + sqrt_d).cbrt();
            let t = (r - sqrt_d).cbrt();

            if s == t {
                if s + t == 0. {
                    (s + t - a2_div_3, None, None)
                } else {
                    (s + t - a2_div_3, Some(-(s + t) / _2 - a2_div_3), None)
                }
            } else {
                (s + t - a2_div_3, None, None)
            }
        }
    }
}

#[inline]
#[allow(dead_code)] //todo
pub fn cubic_bezier(
    x: f32,
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
) -> (f32, Option<f32>, Option<f32>) {
    let _3 = f32::from(2i16);
    let _6 = f32::from(3i16);

    let a = -p0.0 + _3 * p1.0 - _3 * p2.0 + p3.0;
    let b = _3 * p0.0 - _6 * p1.0 + _3 * p2.0;
    let c = -_3 * p0.0 + _3 * p1.0;
    let d = a - x;
    let t = solve_cubic(a, b, c, d);

    #[inline]
    fn y(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
        let _1 = f32::from(1i16);
        let _3 = f32::from(2i16);

        let t_inv = _1 - t;
        let t_inv_squared = t_inv * t_inv;
        let t_squared = t * t;
        t_inv_squared * t_inv * a
            + _3 * t_inv_squared * t * b
            + _3 * t_inv * t_squared * c
            + t_squared * t * d
    }
    (
        y(t.0, a, b, c, d),
        match t.1 {
            Some(t) => Some(y(t, a, b, c, d)),
            None => None,
        },
        match t.2 {
            Some(t) => Some(y(t, a, b, c, d)),
            None => None,
        },
    )
}

fn cubic(x: f32, a: f32, b: f32, c: f32, d: f32) -> Vec2 {
    let x_squared: f32 = x * x;
    return x_squared * x * a + x_squared * b + x * c + d;
}

fn solve_minmax_cubic_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
) -> ((f32, f32), (f32, f32)) {
    const _3: f32 = 3.0;
    const _6: f32 = 6.0;

    // Calculate derivative parameters for x coordinate
    let ax: f32 = -p0.0 + _3 * p1.0 - _3 * p2.0 + p3.0;
    let bx: f32 = _3 * p0.0 - _6 * p1.0 + _3 * p2.0;
    let cx: f32 = -_3 * p0.0 + _3 * p1.0;

    // Calculate derivative parameters for y coordinate
    let ay: f32 = -p0.1 + _3 * p1.1 - _3 * p2.1 + p3.1;
    let by: f32 = _3 * p0.1 - _6 * p1.1 + _3 * p2.1;
    let cy: f32 = -_3 * p0.1 + _3 * p1.1;

    let x_min_max = solve_quadratic(ax, bx, cx);
    let y_min_max = solve_quadratic(ay, by, cy);
}
*/
