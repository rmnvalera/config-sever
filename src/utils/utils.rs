use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use core::hash::Hash;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
