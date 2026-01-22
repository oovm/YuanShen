use ys_types::{
    objects::{ObjectID, TextFile},
    ObjectProxy, BranchProxy,
    YsError, YuanShenObject,
};
use std::path::{Path, PathBuf};

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
        T: for<'de> serde::Deserialize<'de> + Send
    {
        let path = self.store_file(id);
        let bytes = tokio::fs::read(path).await.map_err(|e| YsError::external_error(e))?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn put_typed<T>(&self, obj: &T) -> Result<ObjectID, YsError>
    where
        T: serde::Serialize + YuanShenObject + Send + Sync
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
            tokio::fs::read_to_string(branch_file)
                .await
                .map(|s| s.trim().to_string())
                .map_err(|e| YsError::external_error(e))
        } else {
            Ok("main".to_string())
        }
    }

    async fn set_branch_name(&self, name: &str) -> Result<(), YsError> {
        let branch_file = self.root.join("branch");
        tokio::fs::write(branch_file, name)
            .await
            .map_err(|e| YsError::external_error(e))
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
