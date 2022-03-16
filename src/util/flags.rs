use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Flags(u32);
impl Flags {
    pub const NONE: Self = Flags::new(0x00000000);
    pub const TRANSPARENT: Self = Flags::new(0x1 << 1);
    pub const AUTO_WIDTH: Self = Flags::new(0x1 << 2);
    pub const AUTO_HEIGHT: Self = Flags::new(0x1 << 3);

    const fn new(v: u32) -> Self {
        Flags(v)
    }

    pub fn test(self, flags: Self) -> bool {
        (self | flags) == self
    }
}
impl BitAnd for Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self.0 & rhs.0).into()
    }
}
impl BitOr for Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self.0 | rhs.0).into()
    }
}
impl BitXor for Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}
impl From<u32> for Flags {
    fn from(flags: u32) -> Self {
        Flags(flags)
    }
}