use ys_types::{
    objects::{ObjectID, TextFile},
    traits::ObjectProxy,
    YsError, YuanShenObject,
};
use std::path::{Path, PathBuf};
use tokio::fs::File;

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
        let s = id.hash256.to_string();
        let sub = &s[0..2];
        let filename = &s[2..];
        self.root.join(sub).join(filename)
    }
}

#[async_trait::async_trait]
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

    async fn get_buffer(&self, _: TextFile) -> Result<String, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, _: TextFile, _: &mut File) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, _: &str) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, _file: &mut File) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn get_typed<T: for<'de> serde::Deserialize<'de> + Send>(&self, id: ObjectID) -> Result<T, YsError> {
        let path = self.store_file(id);
        let bytes = tokio::fs::read(path).await.map_err(|e| YsError::external_error(e))?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn put_typed<T: serde::Serialize + YuanShenObject + Send + Sync>(&self, obj: &T) -> Result<ObjectID, YsError> {
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
