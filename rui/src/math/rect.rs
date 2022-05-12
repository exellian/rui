use crate::math;
use crate::math::num::{Max, Min, Num};
use crate::math::{Vec2, Vec4};
use crate::util::Point2D;

#[inline]
pub fn min<T>(lhs: Vec4<T>, rhs: Vec4<T>) -> Vec4<T>
where
    T: Num + Min<Output = T> + Max<Output = T>,
{
    [
        math::max(lhs[0], rhs[0]),
        math::max(lhs[1], rhs[1]),
        math::min(lhs[2], rhs[2]),
        math::min(lhs[3], rhs[3]),
    ]
    .into()
}

#[inline]
pub fn max<T>(lhs: Vec4<T>, rhs: Vec4<T>) -> Vec4<T>
where
    T: Num + Min<Output = T> + Max<Output = T>,
{
    [
        math::min(lhs[0], rhs[0]),
        math::min(lhs[1], rhs[1]),
        math::max(lhs[2], rhs[2]),
        math::max(lhs[3], rhs[3]),
    ]
    .into()
}

#[inline]
pub fn rect<T>(from: Vec2<T>, to: Vec2<T>) -> Vec4<T>
where
    T: Num + Min<Output = T> + Max<Output = T>,
{
    let (mut x_min, mut x_max) = (math::min(from[0], to[0]), math::max(from[0], to[0]));
    let (mut y_min, mut y_max) = (math::min(from[1], to[1]), math::max(from[1], to[1]));
    [x_min, y_min, x_max, y_max].into()
}
