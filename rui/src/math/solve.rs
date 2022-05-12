use crate::math::num::Result::{Defined, Undefined};
use crate::math::num::{Acos, Cbrt, Cos, Field, Result, Sqrt};
use crate::math::{cubic_bezier, Vec2, Vec3};
use std::cmp::{max, min};

#[inline]
fn solve_linear<T>(a: T, b: T) -> Result<T>
where
    T: Field + PartialEq,
{
    if a == T::ZERO {
        if b == T::ZERO {
            return Defined(T::ZERO);
        }
        return Undefined;
    }
    return Defined(-b / a);
}

#[inline]
fn solve_quadratic<T>(a: T, b: T, c: T) -> Vec2<Result<T>>
where
    T: Field + Sqrt + PartialEq + PartialOrd,
{
    if a == T::ZERO {
        let x = solve_linear(b, c);
        return [x, Undefined].into();
    }
    let _0 = T::ZERO;
    let _2 = T::ONE + T::ONE;
    let _4 = _2 + _2;

    let delta0 = _2 * a;
    let delta1 = -b / delta0;
    let discriminant = b * b - _4 * a * c;
    if discriminant < _0 {
        return [Undefined, Undefined].into();
    }
    if discriminant == _0 {
        return [Defined(delta1), Undefined].into();
    }
    let d = (b * b - _4 * a * c).sqrt();
    return [Defined(delta1 + d / delta0), Defined(delta1 - d / delta0)].into();
}

/// Calculates the zero points of a cubic function
#[inline]
pub fn solve_cubic<T>(a: T, b: T, c: T, d: T) -> Vec3<Result<T>>
where
    T: Field + Sqrt + Cbrt + Cos + Acos + PartialEq + PartialOrd,
{
    if a == T::ZERO {
        let res = solve_quadratic(b, c, d);
        return [res[0], res[1], Undefined].into();
    }

    let a2 = b / a;
    let a1 = c / a;
    let a0 = d / a;

    let _2 = T::ONE + T::ONE;
    let _3 = _2 + T::ONE;
    let _4 = _2 + _2;
    let _9 = _3 + _3;
    let _18 = _9 + _9;
    let _27 = _9 + _9 + _9;
    let _54 = _27 + _27;

    let q = (_3 * a1 - a2 * a2) / _9;
    let r = (_9 * a2 * a1 - _27 * a0 - _2 * a2 * a2 * a2) / _54;
    let q3 = q * q * q;
    let d0 = q3 + r * r;
    let a2_div_3 = a2 / _3;

    if d0 < T::ZERO {
        let phi_3 = (r / (-q3).sqrt()).acos() / _3;
        let sqrt_q_2 = _2 * (-q).sqrt();
        return [
            Defined(sqrt_q_2 * (phi_3).cos() - a2_div_3),
            Defined(sqrt_q_2 * (phi_3 - T::TWO_THIRD_PI).cos() - a2_div_3),
            Defined(sqrt_q_2 * (phi_3 + T::TWO_THIRD_PI).cos() - a2_div_3),
        ]
        .into();
    }
    let sqrt_d = (d0).sqrt();
    let s = (r + sqrt_d).cbrt();
    let t = (r - sqrt_d).cbrt();
    let st = s + t;

    if s == t && st != T::ZERO {
        return [
            Defined(st - a2_div_3),
            Defined(-st / _2 - a2_div_3),
            Undefined,
        ]
        .into();
    }
    return [Defined(st - a2_div_3), Undefined, Undefined].into();
}

/// Returns the left upper corner and the right lower corner of the bounding
/// rectangle of the cubic bezier for 0.0 <= t <= 1.0
///
/// # Arguments
/// - p0: The starting point of the cubic bezier
/// - p1: The first helper point of the cubic bezier
/// - p2: The second helper point of the cubic bezier
/// - p3: The end point of the cubic bezier
#[inline]
fn solve_minmax_cubic_bezier<T>(p0: Vec2<T>, p1: Vec2<T>, p2: Vec2<T>, p3: Vec2<T>) -> Vec2<Vec2<T>>
where
    T: Field + Sqrt + PartialEq + Ord,
{
    let _3 = T::ONE + T::ONE + T::ONE;
    let _6 = _3 + _3;

    // Calculate derivative parameters for x coordinate
    let ax = -p0[0] + _3 * p1[0] - _3 * p2[0] + p3[0];
    let bx = _3 * p0[0] - _6 * p1[0] + _3 * p2[0];
    let cx = -_3 * p0[0] + _3 * p1[0];

    // Calculate derivative parameters for y coordinate
    let ay = -p0[1] + _3 * p1[1] - _3 * p2[1] + p3[1];
    let by = _3 * p0[1] - _6 * p1[1] + _3 * p2[1];
    let cy = -_3 * p0[1] + _3 * p1[1];

    let (mut x_min, mut x_max) = (min(p0[0], p3[0]), max(p0[0], p3[0]));
    let (mut y_min, mut y_max) = (min(p0[1], p3[1]), max(p0[1], p3[1]));

    let tx_min_max = solve_quadratic(ax, bx, cx);
    let ty_min_max = solve_quadratic(ay, by, cy);

    for tx_res in tx_min_max {
        if let Defined(tx) = tx_res {
            if tx >= T::ZERO && tx <= T::ONE {
                let p = cubic_bezier(tx, p0, p1, p2, p3);
                x_min = min(p[0], x_min);
                x_max = max(p[0], x_max);
            }
        }
    }
    for ty_res in ty_min_max {
        if let Defined(ty) = ty_res {
            if ty >= T::ZERO && ty <= T::ONE {
                let p = cubic_bezier(ty, p0, p1, p2, p3);
                y_min = min(p[1], y_min);
                y_max = max(p[1], y_max);
            }
        }
    }

    [[x_min, y_min].into(), [x_max, y_max].into()].into()
}
