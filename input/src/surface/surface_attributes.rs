use std::borrow::Cow;

pub struct SurfaceAttributes<'a> {
    title: Option<Cow<'a, str>>
}