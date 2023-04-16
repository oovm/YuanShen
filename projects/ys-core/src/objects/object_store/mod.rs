
use super::*;

pub mod file_system;
pub mod in_memory;



/// 对象储存位置的通用接口，定义了在内存、目录或网络中存储、读取和检查对象的基本操作。
pub trait ObjectStore {
    // 定义了存储过程中可能出现的错误类型。
    type Error: Into<YsError>;

    /// 检查对象是否存在于存储中。
    ///
    /// # 参数
    /// - `id`: 要检查的对象的唯一标识符。
    ///
    /// # 返回值
    /// - `Result<bool, Self::Error>`: 如果对象存在，返回`Result::Ok(true)`；如果不存在或发生错误，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    fn has(&self, id: ObjectID) -> impl Future<Output=Result<bool, Self::Error>> + Send;

    /// 从存储中读取对象。
    ///
    /// # 参数
    /// - `id`: 要读取的对象的唯一标识符。
    ///
    /// # 返回值
    /// - `Result<Option<Vec<u8>>, Self::Error>`: 如果对象存在，返回包含对象数据的`Vec<u8>`的`Result::Ok`；如果对象不存在，返回`Result::Ok(None)`；如果发生错误，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    fn read(&self, id: ObjectID) -> impl Future<Output=Result<Vec<u8>, Self::Error>> + Send;

    /// 将对象插入存储。
    ///
    /// # 参数
    /// - `object`: 要插入存储的对象数据的字节切片。
    ///
    /// # 返回值
    /// - `Result<ObjectID, Self::Error>`: 如果对象成功插入，返回该对象的唯一标识符`ObjectID`的`Result::Ok`；如果插入失败，返回`Result::Err(error)`，其中`error`是`Self::Error`类型。
    fn insert(&mut self, object: &[u8]) -> impl Future<Output=Result<ObjectID, Self::Error>> + Send;
}