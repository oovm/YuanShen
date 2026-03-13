use super::*;
use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};
use uuid::Uuid;

#[derive(Default)]
pub struct ObjectHasher {
    wrapper: blake3::Hasher,
}

impl Debug for ObjectHasher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.wrapper, f)
    }
}

impl Hasher for ObjectHasher {
    fn finish(&self) -> u64 {
        unreachable!()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.wrapper.update(bytes);
    }
}

impl ObjectHasher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.wrapper.update(bytes);
    }

    pub fn finalize(self) -> ObjectID {
        let hash = self.wrapper.finalize();
        let namespace = Uuid::NAMESPACE_DNS;
        ObjectID(Uuid::new_v5(&namespace, hash.as_bytes()))
    }

    pub fn hash<H: Hash>(hashable: H) -> ObjectID {
        let mut hasher = Self::default();
        hashable.hash(&mut hasher);
        hasher.finalize()
    }
}
