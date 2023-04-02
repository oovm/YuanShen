use super::*;


/// 内存对象库
#[derive(Debug, Clone)]
pub struct MemoryObjectStore {
    btree: BTreeMap<ObjectID, Vec<u8>>,
}

impl MemoryObjectStore {
    pub fn new() -> Self {
        Self {
            btree: BTreeMap::new(),
        }
    }
}

impl ObjectStore for MemoryObjectStore {
    type Error = Infallible;

    fn has(&self, id: ObjectID) -> Result<bool, Self::Error> {
        Ok(self.btree.contains_key(&id))
    }

    fn read(&self, id: ObjectID) -> Result<Option<Vec<u8>>, Self::Error> {
        match self.btree.get(&id) {
            Some(v) => Ok(Some(v.clone())),
            None => Ok(None),
        }
    }

    fn insert(&mut self, object: &[u8]) -> Result<ObjectID, Self::Error> {
        let id: ObjectID = object.into();
        match self.btree.entry(id) {
            Entry::Vacant(v) => {
                v.insert(object.into());
                Ok(id)
            }
            Entry::Occupied(_o) => Ok(id),
        }
    }
}

#[test]
fn test_memory_object_store() {
    let mut store = MemoryObjectStore::new();
    store.insert(b"hello, world").unwrap();
    let b: &[u8] = b"hello, world";
    assert!(store.has(b.into()).unwrap());
    assert_eq!(store.read(b.into()).unwrap(), Some(Vec::from(b)));
}
