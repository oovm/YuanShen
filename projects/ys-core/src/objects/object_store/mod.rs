use super::*;
use serde::de::DeserializeOwned;

pub mod file_system;
pub mod in_memory;



/// 对象储存位置的通用接口，定义了在内存、目录或网络中存储、读取和检查对象的基本操作。
#[allow(async_fn_in_trait)]
pub trait ObjectStore {
    /// 检查对象是否存在于存储中。
    ///
    /// # 参数
    /// - `id`: 要检查的对象的唯一标识符。
    ///
    /// # 返回值
    /// - `Result<bool, Self::Error>`: 如果对象存在，返回`Result::Ok(true)`；如果不存在或发生错误，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    fn has(&self, id: ObjectID) -> impl Future<Output = Result<bool, YsError>> + Send;

    /// 从存储中读取对象。
    ///
    /// # 参数
    /// - `id`: 要读取的对象的唯一标识符。
    ///
    /// # 返回值
    /// - `Result<Option<Vec<u8>>, Self::Error>`: 如果对象存在，返回包含对象数据的`Vec<u8>`的`Result::Ok`；如果对象不存在，返回`Result::Ok(None)`；如果发生错误，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    fn get(&self, id: ObjectID) -> impl Future<Output = Result<Vec<u8>, YsError>> + Send;

    /// 从存储中读取对象。
    ///
    /// # 参数
    /// - `id`: 要读取的对象的唯一标识符。
    ///
    /// # 返回值
    /// - `Result<Option<Vec<u8>>, Self::Error>`: 如果对象存在，返回包含对象数据的`Vec<u8>`的`Result::Ok`；如果对象不存在，返回`Result::Ok(None)`；如果发生错误，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    async fn get_typed<O>(&self, id: ObjectID) -> Result<O, YsError>
    where
        O: DeserializeOwned,
    {
        let raw = self.get(id).await?;
        Ok(serde_json::from_slice(&raw)?)
    }

    /// 将对象插入存储。
    ///
    /// # 参数
    /// - `object`: 要插入存储的对象数据的字节切片。
    ///
    /// # 返回值
    /// - `Result<ObjectID, Self::Error>`: 如果对象成功插入，返回该对象的唯一标识符`ObjectID`的`Result::Ok`；如果插入失败，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    fn put(&mut self, id: ObjectID, object: &[u8]) -> impl Future<Output = Result<ObjectID, YsError>> + Send;

    /// 将对象插入存储。
    ///
    /// # 参数
    /// - `object`: 要插入存储的对象数据的字节切片。
    ///
    /// # 返回值
    /// - `Result<ObjectID, Self::Error>`: 如果对象成功插入，返回该对象的唯一标识符`ObjectID`的`Result::Ok`；如果插入失败，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    async fn put_typed<I>(&mut self, object: &I) -> Result<ObjectID, YsError>
    where
        I: YuanShenObject,
    {
        let buffer = object.as_bytes();
        let object_id = self.put(object.object_id(), &buffer).await?;
        Ok(object_id)
    }
}

pub trait YuanShenObject: Serialize + Send + Sync + DeserializeOwned {
    fn object_id(&self) -> ObjectID {
        let buffer = self.as_bytes();
        ObjectID { hash256: blake3::hash(&buffer) }
    }
    fn as_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }
}
