use std::hash::Hash;

#[derive(Hash, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SurfaceId(u64);

impl From<u64> for SurfaceId {
    fn from(id: u64) -> Self {
        SurfaceId(id)
    }
}