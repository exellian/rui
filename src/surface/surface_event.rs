use crate::util::Extent;

#[derive(Debug)]
pub enum SurfaceEvent {
    Redraw,
    Resized(Extent)
}