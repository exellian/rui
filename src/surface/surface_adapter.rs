use crate::util::Extent;

pub trait SurfaceAdapter {

    fn inner_size(&self) -> Extent;
}