use super::*;

pub mod file_system;
pub mod in_memory;


/// 对象的储存位置, 可以是内存, 目录, 或者网络
pub trait ObjectStore {
    /// 储存错误
    type Error;
    async fn has(&self, id: ObjectID) -> Result<bool, Self::Error>;

    async fn read(&self, id: ObjectID) -> Result<Option<Vec<u8>>, Self::Error>;

    async fn insert(&mut self, object: &[u8]) -> Result<ObjectID, Self::Error>;
}
