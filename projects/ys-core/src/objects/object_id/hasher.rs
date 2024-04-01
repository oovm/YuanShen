use super::*;

impl Hasher for ObjectHasher {
    fn finish(&self) -> u64 {
        unreachable!()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.wrapper.update(bytes);
    }
}

impl ObjectHasher {
    pub fn finalize(self) -> ObjectID {
        ObjectID { hash256: self.wrapper.finalize() }
    }
}

impl Hash for ObjectID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash256.hash(state)
    }
}