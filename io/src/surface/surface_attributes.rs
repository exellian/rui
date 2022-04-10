use std::borrow::Cow;

pub struct SurfaceAttributes<'a> {
    pub title: Cow<'a, str>
}

impl<'a> SurfaceAttributes<'a> {

    fn new(title: Cow<'a, str>) -> Self {
        SurfaceAttributes {
            title
        }
    }
}

impl<'a> Default for SurfaceAttributes<'a> {
    fn default() -> Self {
        SurfaceAttributes::new(
            "".into()
        )
    }
}