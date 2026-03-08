use std::collections::BTreeMap;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use ys_types::{ObjectID, YsError};
use ys_storage::StorageBackend;
use crate::Driver;

/// Git 版本控制系统驱动程序，用于与 Git 仓库交互
pub struct GitDriver<S> {
    store: S,
}

impl<S> GitDriver<S>
where
    S: StorageBackend + 'static,
{
    /// 创建新的 GitDriver 实例
    ///
    /// # 参数
    /// * `store` - 用于存储数据的后端实现
    ///
    /// # 返回值
    /// 新创建的 GitDriver 实例
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// 递归遍历目录，导入所有文件到存储后端
    ///
    /// # 参数
    /// * `dir_path` - 要遍历的目录路径
    /// * `entries` - 用于存储目录内容的 BTreeMap
    ///
    /// # 返回值
    /// 操作成功返回 Ok(())，失败返回 YsError
    async fn import_directory(
        &self,
        dir_path: &Path,
        entries: &mut BTreeMap<String, ys_types::snapshot::directory::DirectoryEntry>,
    ) -> Result<(), YsError> {
        let dir = read_dir(dir_path)?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().map_err(|_| {
                YsError::invalid_object("文件名包含无效字符")
            })?;

            if file_name == ".git" {
                continue;
            }

            if path.is_dir() {
                let mut child_entries = BTreeMap::new();
                self.import_directory(&path, &mut child_entries).await?;
                let dir_obj = ys_types::snapshot::directory::DirectoryObject {
                    entries: child_entries,
                };
                entries.insert(
                    file_name,
                    ys_types::snapshot::directory::DirectoryEntry::Directory(dir_obj),
                );
            } else if path.is_file() {
                let text_file = self.store.put_string_file(&path).await?;
                entries.insert(
                    file_name,
                    ys_types::snapshot::directory::DirectoryEntry::TextStandalone(text_file),
                );
            }
        }
        Ok(())
    }
}

impl<S> Driver for GitDriver<S>
where
    S: StorageBackend + 'static,
{
    /// 获取驱动名称
    ///
    /// # 返回值
    /// 固定返回 "git"
    fn name(&self) -> &'static str {
        "git"
    }

    /// 克隆远程 Git 仓库到存储后端
    ///
    /// # 参数
    /// * `url` - 远程仓库的 URL
    /// * `_local_path` - 本地路径参数（未使用，保留用于兼容性）
    ///
    /// # 返回值
    /// 操作成功返回 Ok(())，失败返回 YsError
    async fn clone(&self, url: &str, _local_path: &Path) -> Result<(), YsError> {
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        git2::Repository::clone(url, temp_path).map_err(YsError::external_error)?;

        let mut root_entries = BTreeMap::new();
        self.import_directory(temp_path, &mut root_entries).await?;

        Ok(())
    }

    /// 从远程仓库获取最新提交
    ///
    /// # 参数
    /// * `url` - 远程仓库的 URL
    ///
    /// # 返回值
    /// 操作成功返回最新提交的 ObjectID，失败返回 YsError
    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        println!("Fetching Git repository from {}", url);
        Err(YsError::not_implemented("GitDriver::fetch_to_db"))
    }

    /// 将本地提交推送到远程仓库
    ///
    /// # 参数
    /// * `url` - 远程仓库的 URL
    /// * `commit_id` - 要推送的提交 ID
    ///
    /// # 返回值
    /// 操作成功返回 Ok(())，失败返回 YsError
    async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError> {
        println!("Pushing Git commit {} to {}", commit_id, url);
        Err(YsError::not_implemented("GitDriver::push_from_db"))
    }
}
