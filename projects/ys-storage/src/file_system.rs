use std::{
    collections::{BTreeSet, VecDeque},
    path::{Path, PathBuf},
};
use ys_types::{
    BranchProxy, Commit, DirectoryEntry, DirectoryObject, GarbageCollect, ObjectProxy, SnapShotTree, YsError, YuanShenObject,
    objects::{ObjectID, TextFile},
};

/// 本地文件系统对象储存
#[derive(Debug, Clone)]
pub struct LocalDotYuanShen {
    root: PathBuf,
}

impl LocalDotYuanShen {
    pub fn new(root: PathBuf) -> Result<Self, std::io::Error> {
        if !root.exists() {
            std::fs::create_dir_all(&root)?;
        }
        Ok(Self { root })
    }

    fn store_file(&self, id: ObjectID) -> PathBuf {
        let s = id.to_string();
        let sub = &s[0..2];
        let filename = &s[2..];
        self.root.join(sub).join(filename)
    }
}

impl ObjectProxy for LocalDotYuanShen {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        Ok(self.store_file(id).exists())
    }

    async fn get_string(&self, id: TextFile) -> Result<String, YsError> {
        let file = self.store_file(id.file_id);
        tokio::fs::read_to_string(file).await.map_err(|e| YsError::external_error(e))
    }

    async fn get_string_file(&self, id: TextFile, file_path: &Path) -> Result<(), YsError> {
        let src = self.store_file(id.file_id);
        tokio::fs::copy(src, file_path).await.map_err(|e| YsError::external_error(e))?;
        Ok(())
    }

    async fn put_string(&self, text: &str) -> Result<TextFile, YsError> {
        let id = text.object_id();
        let path = self.store_file(id);
        if !path.exists() {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| YsError::external_error(e))?;
            }
            tokio::fs::write(path, text).await.map_err(|e| YsError::external_error(e))?;
        }
        Ok(TextFile { file_id: id })
    }

    async fn put_string_file(&self, file_path: &Path) -> Result<TextFile, YsError> {
        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| YsError::external_error(e))?;
        self.put_string(&content).await
    }

    async fn get_buffer(&self, _: TextFile) -> Result<Vec<u8>, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, _: TextFile, _: &Path) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, _: &[u8]) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, _: &Path) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn get_typed<T>(&self, id: ObjectID) -> Result<T, YsError>
    where
        T: for<'de> serde::Deserialize<'de> + Send,
    {
        let path = self.store_file(id);
        let bytes = tokio::fs::read(path).await.map_err(|e| YsError::external_error(e))?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn put_typed<T>(&self, obj: &T) -> Result<ObjectID, YsError>
    where
        T: serde::Serialize + YuanShenObject + Send + Sync,
    {
        let id = obj.object_id();
        let path = self.store_file(id);
        if !path.exists() {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| YsError::external_error(e))?;
            }
            let bytes = serde_json::to_vec_pretty(obj)?;
            tokio::fs::write(path, bytes).await.map_err(|e| YsError::external_error(e))?;
        }
        Ok(id)
    }
}

impl BranchProxy for LocalDotYuanShen {
    async fn get_branch_name(&self) -> Result<String, YsError> {
        let branch_file = self.root.join("branch");
        if branch_file.exists() {
            tokio::fs::read_to_string(branch_file).await.map(|s| s.trim().to_string()).map_err(|e| YsError::external_error(e))
        }
        else {
            Ok("main".to_string())
        }
    }

    async fn set_branch_name(&self, name: &str) -> Result<(), YsError> {
        let branch_file = self.root.join("branch");
        tokio::fs::write(branch_file, name).await.map_err(|e| YsError::external_error(e))
    }

    async fn get_branch_id(&self, name: &str) -> Result<ObjectID, YsError> {
        let path = self.root.join("branches").join(name);
        let content = tokio::fs::read_to_string(path).await.map_err(|e| YsError::external_error(e))?;
        Ok(serde_json::from_str(&content)?)
    }

    async fn set_branch_id(&self, name: &str, id: ObjectID) -> Result<(), YsError> {
        let path = self.root.join("branches").join(name);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| YsError::external_error(e))?;
        }
        let content = serde_json::to_string_pretty(&id)?;
        tokio::fs::write(path, content).await.map_err(|e| YsError::external_error(e))
    }

    async fn branch_exists(&self, name: &str) -> Result<bool, YsError> {
        Ok(self.root.join("branches").join(name).exists())
    }

    async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError> {
        let branches_dir = self.root.join("branches");
        if !branches_dir.exists() {
            return Ok(vec![]);
        }
        let mut branches = vec![];
        let mut entries = tokio::fs::read_dir(branches_dir).await.map_err(|e| YsError::external_error(e))?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| YsError::external_error(e))? {
            let name = entry.file_name().to_string_lossy().to_string();
            let id = self.get_branch_id(&name).await?;
            branches.push((name, id));
        }
        Ok(branches)
    }
}

impl LocalDotYuanShen {
    /// 收集所有可达对象的 ObjectID
    async fn collect_reachable_objects(&self) -> Result<BTreeSet<ObjectID>, YsError> {
        let mut reachable = BTreeSet::new();
        let mut queue = VecDeque::new();

        let branches = self.list_branches().await?;
        for (_name, tip) in branches {
            queue.push_back(tip);
        }

        while let Some(object_id) = queue.pop_front() {
            if reachable.contains(&object_id) {
                continue;
            }
            reachable.insert(object_id);

            if let Ok(commit) = self.get_typed::<Commit>(object_id).await {
                queue.push_back(commit.tree);
                for parent in commit.parents {
                    queue.push_back(parent);
                }
            }
            else if let Ok(tree) = self.get_typed::<SnapShotTree>(object_id).await {
                self.collect_tree_entries(&tree.root, &mut reachable, &mut queue).await;
            }
            else if let Ok(directory) = self.get_typed::<DirectoryObject>(object_id).await {
                self.collect_tree_entries(&directory.entries, &mut reachable, &mut queue).await;
            }
        }

        Ok(reachable)
    }

    /// 从目录条目中收集对象
    async fn collect_tree_entries(
        &self,
        entries: &std::collections::BTreeMap<String, DirectoryEntry>,
        reachable: &mut BTreeSet<ObjectID>,
        queue: &mut VecDeque<ObjectID>,
    ) {
        for entry in entries.values() {
            match entry {
                DirectoryEntry::Directory(dir) => {
                    for (_, child_entry) in &dir.entries {
                        self.collect_entry_object(child_entry, reachable, queue).await;
                    }
                }
                DirectoryEntry::TextStandalone(text_file) => {
                    if !reachable.contains(&text_file.file_id) {
                        reachable.insert(text_file.file_id);
                    }
                }
                DirectoryEntry::TextIncremental(_) => {}
                DirectoryEntry::Subtree(subtree) => {
                    queue.push_back(subtree.id);
                }
            }
        }
    }

    /// 收集单个目录条目引用的对象
    async fn collect_entry_object(
        &self,
        entry: &DirectoryEntry,
        reachable: &mut BTreeSet<ObjectID>,
        queue: &mut VecDeque<ObjectID>,
    ) {
        match entry {
            DirectoryEntry::Directory(_) => {}
            DirectoryEntry::TextStandalone(text_file) => {
                if !reachable.contains(&text_file.file_id) {
                    reachable.insert(text_file.file_id);
                }
            }
            DirectoryEntry::TextIncremental(_) => {}
            DirectoryEntry::Subtree(subtree) => {
                queue.push_back(subtree.id);
            }
        }
    }

    /// 列出存储中的所有对象
    async fn list_all_objects(&self) -> Result<Vec<ObjectID>, YsError> {
        let mut objects = Vec::new();
        let objects_dir = self.root.clone();

        if !objects_dir.exists() {
            return Ok(objects);
        }

        let mut entries = tokio::fs::read_dir(objects_dir).await.map_err(|e| YsError::external_error(e))?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| YsError::external_error(e))? {
            let path = entry.path();
            if path.is_dir() {
                let sub_dir_name = entry.file_name().to_string_lossy().to_string();
                if sub_dir_name.len() == 2 && sub_dir_name.chars().all(|c| c.is_ascii_hexdigit()) {
                    let mut sub_entries = tokio::fs::read_dir(path).await.map_err(|e| YsError::external_error(e))?;
                    while let Some(sub_entry) = sub_entries.next_entry().await.map_err(|e| YsError::external_error(e))? {
                        let file_name = sub_entry.file_name().to_string_lossy().to_string();
                        let full_id_str = format!("{}{}", sub_dir_name, file_name);
                        if let Ok(object_id) = full_id_str.parse() {
                            objects.push(object_id);
                        }
                    }
                }
            }
        }

        Ok(objects)
    }

    /// 删除指定的对象
    async fn delete_object(&self, id: ObjectID) -> Result<bool, YsError> {
        let path = self.store_file(id);
        if path.exists() {
            tokio::fs::remove_file(path.clone()).await.map_err(|e| YsError::external_error(e))?;
            if let Some(parent) = path.parent() {
                if let Ok(mut entries) = tokio::fs::read_dir(parent).await {
                    let has_entries = entries.next_entry().await.is_ok_and(|e| e.is_some());
                    if !has_entries {
                        let _ = tokio::fs::remove_dir(parent).await;
                    }
                }
            }
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}

impl GarbageCollect for LocalDotYuanShen {
    /// 执行垃圾收集，删除所有不可达的对象
    async fn garbage_collect(&self) -> Result<usize, YsError> {
        let reachable = self.collect_reachable_objects().await?;
        let all_objects = self.list_all_objects().await?;

        let mut deleted_count = 0;
        for object_id in all_objects {
            if !reachable.contains(&object_id) {
                if self.delete_object(object_id).await? {
                    deleted_count += 1;
                }
            }
        }

        Ok(deleted_count)
    }
}
