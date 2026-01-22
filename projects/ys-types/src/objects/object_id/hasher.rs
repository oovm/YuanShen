use super::*;

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

impl Hash for ObjectID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash256.hash(state)
    }
}

impl ObjectHasher {
    pub fn hash<H: Hash>(hashable: H) -> ObjectID {
        let mut hasher = Self::default();
        hashable.hash(&mut hasher);
        hasher.finalize()
    }
    pub fn finalize(self) -> ObjectID {
        ObjectID { hash256: self.wrapper.finalize() }
    }
}
