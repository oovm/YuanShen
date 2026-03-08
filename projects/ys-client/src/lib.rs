use std::path::Path;
use ys_driver::Driver;
use ys_storage::StorageBackend;
use ys_types::{ObjectID, YsError};

/// Client 是 YuanShen 系统的客户端入口
///
/// Client 负责协调驱动（Driver）和存储（StorageBackend），提供版本控制操作的
/// 高层接口。它支持与不同的版本控制系统（如 Git、SVN、P4）进行交互。
pub struct Client<S, D>
where
    S: StorageBackend,
    D: Driver,
{
    storage: S,
    driver: D,
}

impl<S, D> Client<S, D>
where
    S: StorageBackend,
    D: Driver,
{
    /// 创建一个新的 Client 实例
    ///
    /// # 参数
    /// - `storage`: 存储后端实现，用于持久化数据
    /// - `driver`: 驱动实现，用于与特定版本控制系统交互
    ///
    /// # 返回值
    /// 返回一个新的 Client 实例
    pub fn new(storage: S, driver: D) -> Self {
        Self { storage, driver }
    }

    /// 获取当前使用的驱动名称
    ///
    /// # 返回值
    /// 返回驱动的名称，例如 "git"、"svn" 或 "p4"
    pub fn driver_name(&self) -> &'static str {
        self.driver.name()
    }

    /// 克隆远程仓库到本地路径
    ///
    /// # 参数
    /// - `url`: 远程仓库的 URL
    /// - `local_path`: 本地存储路径
    ///
    /// # 错误
    /// 当克隆失败时返回 YsError
    pub async fn clone(&self, url: &str, local_path: &Path) -> Result<(), YsError> {
        self.driver.clone(url, local_path).await
    }

    /// 从远程仓库获取最新的提交 ID
    ///
    /// # 参数
    /// - `url`: 远程仓库的 URL
    ///
    /// # 返回值
    /// 返回最新的提交 ID
    ///
    /// # 错误
    /// 当获取失败时返回 YsError
    pub async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        self.driver.fetch(url).await
    }

    /// 将本地的提交推送到远程仓库
    ///
    /// # 参数
    /// - `url`: 远程仓库的 URL
    /// - `commit_id`: 要推送的提交 ID
    ///
    /// # 错误
    /// 当推送失败时返回 YsError
    pub async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError> {
        self.driver.push(url, commit_id).await
    }

    /// 获取存储后端的引用
    ///
    /// # 返回值
    /// 返回存储后端的不可变引用
    pub fn storage(&self) -> &S {
        &self.storage
    }

    /// 获取存储后端的可变引用
    ///
    /// # 返回值
    /// 返回存储后端的可变引用
    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }

    /// 获取驱动的引用
    ///
    /// # 返回值
    /// 返回驱动的不可变引用
    pub fn driver(&self) -> &D {
        &self.driver
    }

    /// 获取驱动的可变引用
    ///
    /// # 返回值
    /// 返回驱动的可变引用
    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }
}
