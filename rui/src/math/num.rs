use crate::math::num::Result::{Defined, Undefined};
use std::ops::{AddAssign, Div, DivAssign, MulAssign, Neg, Sub, SubAssign};

pub trait Num: Copy + Sized {}

pub trait Integer: Num + Eq + Ord {}

pub trait Add: Sized + std::ops::Add<Output = Self> + AddAssign {}
pub trait Mul: Sized + std::ops::Mul<Output = Self> + MulAssign {}
pub trait MulInv: Sized + Div<Output = Self> + DivAssign {}
pub trait AddInv: Sized + Neg<Output = Self> + Sub<Output = Self> + SubAssign {}

pub trait Ring: Num + Add + AddInv + Mul {
    const ONE: Self;
    const ZERO: Self;
}

pub trait Field: Ring + MulInv {
    const TWO_THIRD_PI: Self;
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

pub trait Cbrt {
    fn cbrt(self) -> Self;
}
pub trait Cos {
    fn cos(self) -> Self;
}
pub trait Acos {
    fn acos(self) -> Self;
}

pub trait Min<RHS = Self> {
    type Output;
    fn min(self, rhs: RHS) -> Self::Output;
}

pub trait Max<RHS = Self> {
    type Output;
    fn max(self, rhs: RHS) -> Self::Output;
}

#[derive(Copy)]
pub enum Result<T> {
    Defined(T),
    Undefined,
}

impl Num for f32 {}
impl Add for f32 {}
impl AddInv for f32 {}
impl Mul for f32 {}
impl MulInv for f32 {}
impl Ring for f32 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}
impl Field for f32 {
    const TWO_THIRD_PI: Self = 2.09439510239319526263557236234191805;
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Cbrt for f32 {
    fn cbrt(self) -> Self {
        self.cbrt()
    }
}
impl Cos for f32 {
    fn cos(self) -> Self {
        self.cos()
    }
}
impl Acos for f32 {
    fn acos(self) -> Self {
        self.acos()
    }
}
impl Min for f32 {
    type Output = f32;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for f32 {
    type Output = f32;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for f64 {}
impl Add for f64 {}
impl AddInv for f64 {}
impl Mul for f64 {}
impl MulInv for f64 {}
impl Ring for f64 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}
impl Field for f64 {
    const TWO_THIRD_PI: Self = 2.09439510239319526263557236234191805;
}
impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Cbrt for f64 {
    fn cbrt(self) -> Self {
        self.cbrt()
    }
}
impl Cos for f64 {
    fn cos(self) -> Self {
        self.cos()
    }
}
impl Acos for f64 {
    fn acos(self) -> Self {
        self.acos()
    }
}
impl Min for f64 {
    type Output = f64;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for f64 {
    type Output = f64;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Add for isize {}
impl AddInv for isize {}
impl Mul for isize {}
impl Ring for isize {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl Min for isize {
    type Output = isize;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for isize {
    type Output = isize;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Add for i8 {}
impl AddInv for i8 {}
impl Mul for i8 {}
impl Ring for i8 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl Min for i8 {
    type Output = i8;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for i8 {
    type Output = i8;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Add for i16 {}
impl AddInv for i16 {}
impl Mul for i16 {}
impl Ring for i16 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl Min for i16 {
    type Output = i16;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for i16 {
    type Output = i16;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Add for i32 {}
impl AddInv for i32 {}
impl Mul for i32 {}
impl Ring for i32 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl Min for i32 {
    type Output = i32;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for i32 {
    type Output = i32;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Add for i64 {}
impl AddInv for i64 {}
impl Mul for i64 {}
impl Ring for i64 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl Min for i64 {
    type Output = i64;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for i64 {
    type Output = i64;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Add for i128 {}
impl AddInv for i128 {}
impl Mul for i128 {}
impl Ring for i128 {
    const ONE: Self = 1;
    const ZERO: Self = 0;
}
impl Min for i128 {
    type Output = i128;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for i128 {
    type Output = i128;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for isize {}
impl Integer for isize {}

impl Num for usize {}
impl Integer for usize {}
impl Min for usize {
    type Output = usize;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for usize {
    type Output = usize;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for i8 {}
impl Integer for i8 {}

impl Num for u8 {}
impl Integer for u8 {}
impl Min for u8 {
    type Output = u8;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for u8 {
    type Output = u8;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for i16 {}
impl Integer for i16 {}

impl Num for u16 {}
impl Integer for u16 {}
impl Min for u16 {
    type Output = u16;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for u16 {
    type Output = u16;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for i32 {}
impl Integer for i32 {}

impl Num for u32 {}
impl Integer for u32 {}
impl Min for u32 {
    type Output = u32;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for u32 {
    type Output = u32;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for i64 {}
impl Integer for i64 {}

impl Num for u64 {}
impl Integer for u64 {}
impl Min for u64 {
    type Output = u64;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for u64 {
    type Output = u64;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl Num for i128 {}
impl Integer for i128 {}

impl Num for u128 {}
impl Integer for u128 {}
impl Min for u128 {
    type Output = u128;

    fn min(self, rhs: Self) -> Self::Output {
        return if self < rhs { self } else { rhs };
    }
}
impl Max for u128 {
    type Output = u128;

    fn max(self, rhs: Self) -> Self::Output {
        return if self > rhs { self } else { rhs };
    }
}

impl<T> Min for Result<T>
where
    T: Min<Output = T>,
{
    type Output = Self;

    fn min(self, rhs: Self) -> Self::Output {
        match self {
            Defined(x) => match rhs {
                Defined(y) => Defined(x.min(y)),
                Undefined => Undefined,
            },
            Undefined => Undefined,
        }
    }
}
impl<T> Max for Result<T>
where
    T: Max<Output = T>,
{
    type Output = Self;

    fn max(self, rhs: Self) -> Self::Output {
        match self {
            Defined(x) => match rhs {
                Defined(y) => Defined(x.max(y)),
                Undefined => Undefined,
            },
            Undefined => Undefined,
        }
    }
}

impl<T> Clone for Result<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Defined(x) => Defined(x.clone()),
            Undefined => Undefined,
        }
    }
}

impl<T> std::ops::Add for Result<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Result<T>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Defined(x) => match rhs {
                Defined(y) => Defined(x + y),
                Undefined => Undefined,
            },
            Undefined => Undefined,
        }
    }
}
impl<T> AddAssign for Result<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Defined(x) => match rhs {
                Defined(y) => x.add_assign(y),
                Undefined => {}
            },
            Undefined => {}
        }
    }
}

impl<T> Neg for Result<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Defined(x) => Defined(-x),
            Undefined => Undefined,
        }
    }
}

impl<T> Sub for Result<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Defined(x) => match rhs {
                Defined(y) => Defined(x - y),
                Undefined => Undefined,
            },
            Undefined => Undefined,
        }
    }
}

impl<T> SubAssign for Result<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Defined(x) => match rhs {
                Defined(y) => x.sub_assign(y),
                Undefined => {}
            },
            Undefined => {}
        }
    }
}
impl<T> std::ops::Mul for Result<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Defined(x) => match rhs {
                Defined(y) => Defined(x * y),
                Undefined => Undefined,
            },
            Undefined => Undefined,
        }
    }
}

impl<T> std::ops::MulAssign for Result<T>
where
    T: std::ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        match self {
            Defined(x) => match rhs {
                Defined(y) => x.mul_assign(y),
                Undefined => {}
            },
            Undefined => {}
        }
    }
}

impl<T> std::ops::Div for Result<T>
where
    T: std::ops::Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Defined(x) => match rhs {
                Defined(y) => Defined(x / y),
                Undefined => Undefined,
            },
            Undefined => Undefined,
        }
    }
}

impl<T> std::ops::DivAssign for Result<T>
where
    T: std::ops::DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        match self {
            Defined(x) => match rhs {
                Defined(y) => x.div_assign(y),
                Undefined => {}
            },
            Undefined => {}
        }
    }
}

impl<T> Num for Result<T>
where
    Self: Sized,
    T: Num,
{
}
impl<T> Add for Result<T>
where
    Self: Sized,
    T: Add,
{
}
impl<T> AddInv for Result<T> where T: AddInv {}
impl<T> Mul for Result<T> where T: Mul {}
impl<T> MulInv for Result<T> where T: MulInv {}
impl<T> Ring for Result<T>
where
    T: Ring,
{
    const ONE: Self = Defined(T::ONE);
    const ZERO: Self = Defined(T::ZERO);
}
impl<T> Field for Result<T>
where
    T: Field,
{
    const TWO_THIRD_PI: Self = Defined(T::TWO_THIRD_PI);
}
