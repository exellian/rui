use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn id<T>(h: &T) -> u64 where T: Hash {
    let mut hasher = DefaultHasher::new();
    h.hash(&mut hasher);
    hasher.finish()
}