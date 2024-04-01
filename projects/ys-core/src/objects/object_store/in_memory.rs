use super::*;


/// 内存对象库
#[derive(Debug, Clone)]
pub struct MemoryObjectStore {
    btree: BTreeMap<ObjectID, Vec<u8>>,
}

impl MemoryObjectStore {
    /// 创建一个内存对象储存库, 从内存中获取对象永远不会失败
    pub fn new() -> Self {
        Self {
            btree: BTreeMap::new(),
        }
    }
}

impl ObjectStore for MemoryObjectStore {

    async fn has(&self, _: ObjectID) -> Result<bool, YsError> {
        return Ok(true);
    }

    async fn get(&self, id: ObjectID) -> Result<Vec<u8>, YsError> {
        match self.btree.get(&id) {
            Some(v) => Ok(v.clone()),
            None => Ok(vec![]),
        }
    }

    async fn put(&mut self, id: ObjectID, object: &[u8]) -> Result<ObjectID, YsError> {
        match self.btree.entry(id) {
            // id 不存在, 插入新对象
            Entry::Vacant(v) => {
                v.insert(object.into());
                Ok(id)
            }
            // id 已经存在, 同一个对象只会有一个 id, 无需重复插入
            Entry::Occupied(_) => Ok(id),
        }
    }
}
