use std::path::Path;
use ys_types::{ObjectID, YsError};
use ys_storage::StorageBackend;
use ys_driver::Driver;

pub struct GitDriver<S> {
    store: S,
}

impl<S> GitDriver<S> 
where 
    S: StorageBackend + 'static
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> Driver for GitDriver<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "git"
    }

    async fn clone(&self, url: &str, _local_path: &Path) -> Result<(), YsError> {
        // 在基于 DB 的管理中，clone 可能意味着从远程获取所有对象并存入 DB
        println!("Cloning Git repository from {} into database", url);
        // TODO: 实现 Git 协议的 fetch 逻辑并存入 self.store
        Err(YsError::not_implemented("GitDriver::clone_to_db"))
    }

    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        println!("Fetching Git repository from {}", url);
        // TODO: 实现 Git 协议的 fetch 逻辑，增量更新 self.store
        Err(YsError::not_implemented("GitDriver::fetch_to_db"))
    }

    async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError> {
        println!("Pushing Git commit {} to {}", commit_id, url);
        // TODO: 实现将 self.store 中的对象按 Git 协议推送到远程
        Err(YsError::not_implemented("GitDriver::push_from_db"))
    }
}
