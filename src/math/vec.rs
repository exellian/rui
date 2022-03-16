macro_rules! VecN {
    ($name: ident, $n: expr, $typ: ident, $( $x:ident ),+ ) => {
        pub struct $name($($x),+);

        impl From<[$typ;$n]> for $name {
            fn from(a: [$typ;$n]) -> Self {
                a.into()
            }
        }
    };
}
VecN!(Vec2, 2, f32, f32, f32);
VecN!(Vec3, 3, f32, f32, f32, f32);
VecN!(Vec4, 4, f32, f32, f32, f32, f32);
VecN!(UVec2, 2, usize, usize, usize);
VecN!(UVec3, 3, usize, usize, usize, usize);
VecN!(UVec4, 4, usize, usize, usize, usize, usize);
VecN!(IVec2, 2, isize, isize, isize);
VecN!(IVec3, 3, isize, isize, isize, isize);
VecN!(IVec4, 4, isize, isize, isize, isize, isize);
VecN!(BVec2, 2, bool, bool, bool);
VecN!(BVec3, 3, bool, bool, bool, bool);
VecN!(BVec4, 4, bool, bool, bool, bool, bool);