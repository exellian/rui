use std::borrow::Cow;
use util::Extent;

pub struct Attributes<'a> {
    pub title: Cow<'a, str>,
    pub size: Extent
}

impl<'a> Attributes<'a> {

    pub fn new(title: Cow<'a, str>, size: Extent) -> Self {
        Attributes {
            title,
            size
        }
    }
}