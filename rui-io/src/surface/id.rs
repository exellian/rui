use std::hash::Hash;

#[derive(Hash, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Id(u64);

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Id(id)
    }
}