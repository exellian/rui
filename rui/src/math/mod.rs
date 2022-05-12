mod num;
mod solve;
mod vec;

use crate::math::num::{Ring, Sqrt};
use std::ops::{Add, Mul};
pub use vec::*;

#[inline]
pub fn linear<T, const SIZE: usize>(
    x: Vector<T, SIZE>,
    a: Vector<T, SIZE>,
    b: Vector<T, SIZE>,
) -> Vector<T, SIZE>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    x * a + b
}

#[inline]
pub fn quadratic<T, const SIZE: usize>(
    x: Vector<T, SIZE>,
    a: Vector<T, SIZE>,
    b: Vector<T, SIZE>,
    c: Vector<T, SIZE>,
) -> Vector<T, SIZE>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    x * x * a + x * b + c
}

#[inline]
pub fn cubic<T, const SIZE: usize>(
    x: Vector<T, SIZE>,
    a: Vector<T, SIZE>,
    b: Vector<T, SIZE>,
    c: Vector<T, SIZE>,
    d: Vector<T, SIZE>,
) -> Vector<T, SIZE>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    let x_squared = x * x;
    x_squared * x * a + x_squared * b + x * c + d
}

#[inline]
pub fn cubic_bezier<T>(t: T, b0: Vec2<T>, b1: Vec2<T>, b2: Vec2<T>, b3: Vec2<T>) -> Vec2<T>
where
    T: Ring,
{
    let _3: T = T::ONE + T::ONE + T::ONE;
    let _6: T = _3 + _3;

    let a = -b0 * _3 * b1 - b2 * _3 + b3;
    let b = b0 * _3 - b1 * _6 + b2 * _3;
    let c = -b0 * _3 + b1 * _3;
    let d = b0;
    cubic([t, t].into(), a, b, c, d)
}
