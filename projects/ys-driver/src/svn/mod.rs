use std::collections::BTreeMap;
use std::fs::{read_dir, create_dir_all, write};
use std::path::Path;
use std::process::Command;
use ys_types::{DirectoryEntry, ObjectID, YsError, Commit, SnapShotTree};
use ys_storage::StorageBackend;
use crate::Driver;

/// SVN 版本控制系统驱动程序，用于与 SVN 仓库交互
pub struct SvnDriver<S> {
    store: S,
}

impl<S> SvnDriver<S>
where
    S: StorageBackend + 'static,
{
    /// 创建新的 SvnDriver 实例
    ///
    /// # 参数
    /// * `store` - 用于存储数据的后端实现
    ///
    /// # 返回值
    /// 新创建的 SvnDriver 实例
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
    fn import_directory_sync(
        &self,
        dir_path: &Path,
        entries: &mut BTreeMap<String, DirectoryEntry>,
    ) -> Result<(), YsError> {
        let dir = read_dir(dir_path)?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().map_err(|_| {
                YsError::invalid_object("文件名包含无效字符")
            })?;

            if file_name == ".svn" {
                continue;
            }

            if path.is_dir() {
                let mut child_entries = BTreeMap::new();
                self.import_directory_sync(&path, &mut child_entries)?;
                let dir_obj = ys_types::DirectoryObject {
                    entries: child_entries,
                };
                entries.insert(
                    file_name,
                    DirectoryEntry::Directory(dir_obj),
                );
            } else if path.is_file() {
                let rt = tokio::runtime::Runtime::new()?;
                let text_file = rt.block_on(self.store.put_string_file(&path))?;
                entries.insert(
                    file_name,
                    DirectoryEntry::TextStandalone(text_file),
                );
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

    /// 执行 SVN 命令
    ///
    /// # 参数
    /// * `args` - SVN 命令参数
    /// * `working_dir` - 工作目录
    ///
    /// # 返回值
    /// 命令执行成功返回 Ok(())，失败返回 YsError
    fn run_svn_command(
        args: &[&str],
        working_dir: Option<&Path>,
    ) -> Result<String, YsError> {
        let mut cmd = Command::new("svn");
        cmd.args(args);

        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ys_types::YsErrorKind::External {
                message: format!("SVN 命令执行失败: {}", error_msg),
            }.into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

impl<S> Driver for SvnDriver<S>
where
    S: StorageBackend + 'static,
{
    /// 获取驱动名称
    ///
    /// # 返回值
    /// 固定返回 "svn"
    fn name(&self) -> &'static str {
        "svn"
    }

    /// 克隆远程 SVN 仓库到存储后端
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

        Self::run_svn_command(&["checkout", url, temp_path.to_str().unwrap()], None)?;

        let mut root_entries = BTreeMap::new();
        self.import_directory_sync(temp_path, &mut root_entries)?;

        Ok(())
    }

    /// 从远程仓库获取最新版本
    ///
    /// # 参数
    /// * `url` - 远程仓库的 URL
    ///
    /// # 返回值
    /// 操作成功返回最新版本的 ObjectID，失败返回 YsError
    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        let stdout = Self::run_svn_command(&["info", url], None)?;

        let revision = stdout
            .lines()
            .find(|line| line.starts_with("Last Changed Rev:"))
            .and_then(|line| line.split(": ").nth(1))
            .and_then(|rev| rev.parse::<u64>().ok())
            .ok_or_else(|| YsError::invalid_object("无法解析 SVN 版本号"))?;

        let mut buffer = Vec::new();
        buffer.extend_from_slice(revision.to_string().as_bytes());
        buffer.extend_from_slice(url.as_bytes());
        let object_id = ObjectID::from(buffer.as_slice());

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

        Self::run_svn_command(&["checkout", url, temp_path.to_str().unwrap()], None)?;

        let commit: Commit = self.store.get_typed(commit_id).await?;
        let tree: SnapShotTree = self.store.get_typed(commit.tree).await?;

        self.export_directory(&tree.root, temp_path).await?;

        Self::run_svn_command(&["add", "--force", "."], Some(temp_path))?;

        let message = &commit.extra.message;
        Self::run_svn_command(&["commit", "-m", message], Some(temp_path))?;

        Ok(())
    }
}
