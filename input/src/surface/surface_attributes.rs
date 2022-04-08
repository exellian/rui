use std::borrow::Cow;

pub struct SurfaceAttributes<'a> {
    title: Option<Cow<'a, str>>
}

impl<'a> SurfaceAttributes<'a> {

    fn new(title: Option<Cow<'a, str>>) -> Self {
        SurfaceAttributes {
            title
        }
    }
}

impl<'a> Default for SurfaceAttributes<'a> {
    fn default() -> Self {
        SurfaceAttributes::new(
            None
        )
    }
}