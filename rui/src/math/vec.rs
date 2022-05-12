use crate::math::num::{AddInv, Field, Max, Min, MulInv, Num, Ring};
use crate::math::{max, min};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
    DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::ptr;

pub type Vec2<T> = Vector<T, 2>;
pub type Vec3<T> = Vector<T, 3>;
pub type Vec4<T> = Vector<T, 4>;

#[derive(Copy, Debug)]
pub struct Vector<T, const SIZE: usize>([T; SIZE]);

impl<T, const SIZE: usize> Min for Vector<T, SIZE>
where
    T: Min<Output = T>,
{
    type Output = Self;

    fn min(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    min(ptr::read(&self.0[index]), ptr::read(&rhs.0[index])),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> Max for Vector<T, SIZE>
where
    T: Max<Output = T>,
{
    type Output = Self;

    fn max(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    max(ptr::read(&self.0[index]), ptr::read(&rhs.0[index])),
                );
            }
        }
        self
    }
}

impl<T, const SIZE: usize> Index<usize> for Vector<T, SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, const SIZE: usize> IndexMut<usize> for Vector<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T, const SIZE: usize> Vector<T, SIZE> {
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.0.iter_mut()
    }
}
impl<T, const SIZE: usize> IntoIterator for Vector<T, SIZE> {
    type Item = T;
    type IntoIter = <[T; SIZE] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const SIZE: usize> Num for Vector<T, SIZE> where Self: Copy {}
impl<T, const SIZE: usize> crate::math::num::Add for Vector<T, SIZE> where
    T: Add<Output = T> + AddAssign
{
}
impl<T, const SIZE: usize> AddInv for Vector<T, SIZE> where
    T: Neg<Output = T> + Sub<Output = T> + SubAssign
{
}
impl<T, const SIZE: usize> crate::math::num::Mul for Vector<T, SIZE> where
    T: Mul<Output = T> + MulAssign
{
}
impl<T, const SIZE: usize> MulInv for Vector<T, SIZE> where T: Div<Output = T> + DivAssign {}

// Cargo check bug shows error but compiles fine
impl<T, const SIZE: usize> Ring for Vector<T, SIZE>
where
    T: Ring,
{
    const ONE: Self = Vector([T::ONE; SIZE]);
    const ZERO: Self = Vector([T::ZERO; SIZE]);
}

// Cargo check bug shows error but compiles fine
impl<T, const SIZE: usize> Field for Vector<T, SIZE>
where
    T: Field,
{
    const TWO_THIRD_PI: Self = Vector([T::TWO_THIRD_PI; SIZE]);
}

impl<T, const SIZE: usize> Clone for Vector<T, SIZE>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Vector(self.0.clone())
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for Vector<T, SIZE> {
    fn from(arr: [T; SIZE]) -> Self {
        Vector(arr)
    }
}
impl<T, const SIZE: usize> Into<[T; SIZE]> for Vector<T, SIZE> {
    fn into(self) -> [T; SIZE] {
        self.0
    }
}
impl<T, const SIZE: usize> Deref for Vector<T, SIZE> {
    type Target = [T; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const SIZE: usize> DerefMut for Vector<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T, const SIZE: usize> Add for Vector<T, SIZE>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) + ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> AddAssign for Vector<T, SIZE>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] += ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> Sub for Vector<T, SIZE>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) - ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> SubAssign for Vector<T, SIZE>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] -= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> Mul<T> for Vector<T, SIZE>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(&mut self.0[index], ptr::read(&self.0[index]) * rhs);
            }
        }
        self
    }
}
impl<T, const SIZE: usize> Mul for Vector<T, SIZE>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) * ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> MulAssign for Vector<T, SIZE>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] *= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> Div for Vector<T, SIZE>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) / ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> DivAssign for Vector<T, SIZE>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] /= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> BitOr for Vector<T, SIZE>
where
    T: BitOr<Output = T>,
{
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) | ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> BitOrAssign for Vector<T, SIZE>
where
    T: BitOrAssign,
{
    fn bitor_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] |= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> BitAnd for Vector<T, SIZE>
where
    T: BitAnd<Output = T>,
{
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) & ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> BitAndAssign for Vector<T, SIZE>
where
    T: BitAndAssign,
{
    fn bitand_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] &= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> BitXor for Vector<T, SIZE>
where
    T: BitXor<Output = T>,
{
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) ^ ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> BitXorAssign for Vector<T, SIZE>
where
    T: BitXorAssign,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] ^= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> Not for Vector<T, SIZE>
where
    T: Not<Output = T>,
{
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(&mut self.0[index], !ptr::read(&self.0[index]));
            }
        }
        self
    }
}
impl<T, const SIZE: usize> Neg for Vector<T, SIZE>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(&mut self.0[index], -ptr::read(&self.0[index]));
            }
        }
        self
    }
}
impl<T, const SIZE: usize> Rem for Vector<T, SIZE>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) % ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> RemAssign for Vector<T, SIZE>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] %= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> Shl for Vector<T, SIZE>
where
    T: Shl<Output = T>,
{
    type Output = Self;

    fn shl(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) << ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> ShlAssign for Vector<T, SIZE>
where
    T: ShlAssign,
{
    fn shl_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] <<= ptr::read(&rhs.0[index]);
            }
        }
    }
}
impl<T, const SIZE: usize> Shr for Vector<T, SIZE>
where
    T: Shr<Output = T>,
{
    type Output = Self;

    fn shr(mut self, rhs: Self) -> Self::Output {
        for index in 0..SIZE {
            unsafe {
                ptr::replace(
                    &mut self.0[index],
                    ptr::read(&self.0[index]) >> ptr::read(&rhs.0[index]),
                );
            }
        }
        self
    }
}
impl<T, const SIZE: usize> ShrAssign for Vector<T, SIZE>
where
    T: ShrAssign,
{
    fn shr_assign(&mut self, rhs: Self) {
        for index in 0..SIZE {
            unsafe {
                self.0[index] >>= ptr::read(&rhs.0[index]);
            }
        }
    }
}
