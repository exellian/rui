use crate::surface::SurfaceId;
use crate::util::Extent;

pub trait SurfaceAdapter {

    fn inner_size(&self) -> Extent;
    fn id(&self) -> SurfaceId;
    fn request_redraw(&self);
}