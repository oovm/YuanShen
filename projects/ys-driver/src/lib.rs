use std::path::Path;
use ys_types::{ObjectID, YsError};

/// Driver trait 定义了客户端如何与不同的版本控制协议进行交互
pub trait Driver: Send + Sync {
    /// 获取驱动的名称 (例如 "git", "svn", "p4")
    fn name(&self) -> &'static str;

    /// 将远程仓库克隆到本地路径
    async fn clone(&self, url: &str, local_path: &Path) -> Result<(), YsError>;

    /// 从远程仓库获取最新的提交 ID
    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError>;

    /// 将本地的提交推送到远程仓库
    async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError>;
}
