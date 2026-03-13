use crate::Driver;
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, read_dir, write},
    path::Path,
};
use ys_storage::StorageBackend;
use ys_types::{Commit, DirectoryEntry, ObjectID, SnapShotTree, YsError};

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
    fn import_directory_sync(&self, dir_path: &Path, entries: &mut BTreeMap<String, DirectoryEntry>) -> Result<(), YsError> {
        let dir = read_dir(dir_path)?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().map_err(|_| YsError::invalid_object("文件名包含无效字符"))?;

            if file_name == ".git" {
                continue;
            }

            if path.is_dir() {
                let mut child_entries = BTreeMap::new();
                self.import_directory_sync(&path, &mut child_entries)?;
                let dir_obj = ys_types::DirectoryObject { entries: child_entries };
                entries.insert(file_name, DirectoryEntry::Directory(dir_obj));
            }
            else if path.is_file() {
                let rt = tokio::runtime::Runtime::new()?;
                let text_file = rt.block_on(self.store.put_string_file(&path))?;
                entries.insert(file_name, DirectoryEntry::TextStandalone(text_file));
            }
        }
        Ok(())
    }

    /// 递归将目录条目导出到文件系统
    ///
    /// # 参数
    /// * `entries` - 目录条目集合
    /// * `path` - 目标路径
    ///
    /// # 返回值
    /// 操作成功返回 Ok(())，失败返回 YsError
    fn export_directory<'a>(
        &'a self,
        entries: &'a BTreeMap<String, DirectoryEntry>,
        path: &'a Path,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), YsError>> + 'a>> {
        Box::pin(async move {
            create_dir_all(path)?;

            for (name, entry) in entries {
                let entry_path = path.join(name);

                match entry {
                    DirectoryEntry::Directory(dir_obj) => {
                        self.export_directory(&dir_obj.entries, &entry_path).await?;
                    }
                    DirectoryEntry::TextStandalone(text_file) => {
                        let content = self.store.get_string(text_file.clone()).await?;
                        write(&entry_path, content)?;
                    }
                    DirectoryEntry::TextIncremental(_) => {
                        return Err(YsError::not_implemented("TextIncremental 导出尚未实现"));
                    }
                    DirectoryEntry::Subtree(_) => {
                        return Err(YsError::not_implemented("Subtree 导出尚未实现"));
                    }
                }
            }

            Ok(())
        })
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
        self.import_directory_sync(temp_path, &mut root_entries)?;

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
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let repo = git2::Repository::clone(url, temp_path).map_err(YsError::external_error)?;

        let head = repo.head().map_err(YsError::external_error)?;
        let commit = head.peel_to_commit().map_err(YsError::external_error)?;
        let git_oid = commit.id();
        let git_oid_bytes = git_oid.as_bytes();

        let object_id = ObjectID::from(git_oid_bytes);

        Ok(object_id)
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
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let repo = git2::Repository::clone(url, temp_path).map_err(YsError::external_error)?;

        let commit: Commit = self.store.get_typed(commit_id).await?;

        let tree: SnapShotTree = self.store.get_typed(commit.tree).await?;

        self.export_directory(&tree.root, temp_path).await?;

        let mut index = repo.index().map_err(YsError::external_error)?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).map_err(YsError::external_error)?;
        index.write().map_err(YsError::external_error)?;

        let tree_oid = index.write_tree().map_err(YsError::external_error)?;
        let tree = repo.find_tree(tree_oid).map_err(YsError::external_error)?;

        let head = repo.head().map_err(YsError::external_error)?;
        let parent_commit = head.peel_to_commit().map_err(YsError::external_error)?;

        let signature = git2::Signature::now("GitDriver", "gitdriver@example.com").map_err(YsError::external_error)?;

        let message = &commit.extra.message;

        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent_commit])
            .map_err(YsError::external_error)?;

        let mut remote = repo.find_remote("origin").map_err(YsError::external_error)?;
        let mut push_options = git2::PushOptions::new();
        remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_options)).map_err(YsError::external_error)?;

        Ok(())
    }
}
