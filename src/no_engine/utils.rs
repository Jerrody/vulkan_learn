use std::hash::{BuildHasher, Hash, Hasher};

pub fn hash<H: Hash>(hashable: &H) -> u64 {
    let mut hasher = ahash::RandomState::with_seeds(1, 2, 3, 4).build_hasher();
    hashable.hash(&mut hasher);

    hasher.finish()
}
