use std::hash::Hash;
use crate::util::Extent;

pub trait SurfaceAdapter: Hash {

    fn inner_size(&self) -> Extent;
}