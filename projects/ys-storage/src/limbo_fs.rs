use ys_types::{
    objects::{ObjectID, TextFile},
    ObjectProxy, BranchProxy,
    YsError, YuanShenObject,
};
use std::path::{Path, PathBuf};
use tokio::fs::File;
use std::sync::Arc;

/// Limbo + FS 存储方案
/// 使用 Limbo (SQLite) 存储元数据，FS 存储大文件对象
pub struct LimboFsStorage {
    root: PathBuf,
    db: Arc<limbo::Database>,
}

impl LimboFsStorage {
    pub async fn new(root: PathBuf) -> Result<Self, YsError> {
        if !root.exists() {
            tokio::fs::create_dir_all(&root).await.map_err(|e| YsError::external_error(e))?;
        }
        
        let db_path = root.join("metadata.db");
        let db = limbo::Builder::new_local(db_path.to_str().unwrap())
            .build()
            .await
            .map_err(|e| YsError::external_error(e))?;
        
        let conn = db.connect().map_err(|e| YsError::external_error(e))?;
        // 初始化表
        conn.execute("CREATE TABLE IF NOT EXISTS branches (name TEXT, id BLOB)", ())
            .await
            .map_err(|e| YsError::external_error(e))?;
        conn.execute("CREATE TABLE IF NOT EXISTS config (key TEXT, value TEXT)", ())
            .await
            .map_err(|e| YsError::external_error(e))?;

        Ok(Self {
            root,
            db: Arc::new(db),
        })
    }

    fn store_file(&self, id: ObjectID) -> PathBuf {
        let s = id.to_string();
        let sub = &s[0..2];
        let filename = &s[2..];
        self.root.join("objects").join(sub).join(filename)
    }
}

impl ObjectProxy for LimboFsStorage {
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

impl BranchProxy for LimboFsStorage {
    async fn get_branch_name(&self) -> Result<String, YsError> {
        let conn = self.db.connect().map_err(|e| YsError::external_error(e))?;
        let mut rows = conn
            .query("SELECT value FROM config WHERE key = 'current_branch'", ())
            .await
            .map_err(|e| YsError::external_error(e))?;
        if let Some(row) = rows.next().await.map_err(|e| YsError::external_error(e))? {
            let val = row.get_value(0).map_err(|e| YsError::external_error(e))?;
            if let limbo::Value::Text(s) = val {
                return Ok(s);
            }
        }
        Ok("main".to_string())
    }

    async fn set_branch_name(&self, name: &str) -> Result<(), YsError> {
        let conn = self.db.connect().map_err(|e| YsError::external_error(e))?;
        conn.execute(
            "INSERT OR REPLACE INTO config (key, value) VALUES ('current_branch', ?1)",
            [name],
        )
        .await
        .map_err(|e| YsError::external_error(e))?;
        Ok(())
    }

    async fn get_branch_id(&self, name: &str) -> Result<ObjectID, YsError> {
        let conn = self.db.connect().map_err(|e| YsError::external_error(e))?;
        let mut rows = conn
            .query("SELECT id FROM branches WHERE name = ?1", [name])
            .await
            .map_err(|e| YsError::external_error(e))?;
        if let Some(row) = rows.next().await.map_err(|e| YsError::external_error(e))? {
            let val = row.get_value(0).map_err(|e| YsError::external_error(e))?;
            if let limbo::Value::Blob(bytes) = val {
                return ObjectID::from_bytes(&bytes);
            }
        }
        Err(YsError::invalid_object(format!("Branch not found: {}", name)))
    }

    async fn set_branch_id(&self, name: &str, id: ObjectID) -> Result<(), YsError> {
        let conn = self.db.connect().map_err(|e| YsError::external_error(e))?;
        conn.execute(
            "INSERT OR REPLACE INTO branches (name, id) VALUES (?1, ?2)",
            (name, id.as_bytes().to_vec()),
        )
        .await
        .map_err(|e| YsError::external_error(e))?;
        Ok(())
    }

    async fn branch_exists(&self, name: &str) -> Result<bool, YsError> {
        let conn = self.db.connect().map_err(|e| YsError::external_error(e))?;
        let mut rows = conn
            .query("SELECT 1 FROM branches WHERE name = ?1", [name])
            .await
            .map_err(|e| YsError::external_error(e))?;
        Ok(rows
            .next()
            .await
            .map_err(|e| YsError::external_error(e))?
            .is_some())
    }

    async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError> {
        let conn = self.db.connect().map_err(|e| YsError::external_error(e))?;
        let mut rows = conn
            .query("SELECT name, id FROM branches", ())
            .await
            .map_err(|e| YsError::external_error(e))?;
        let mut branches = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| YsError::external_error(e))? {
            let name_val = row.get_value(0).map_err(|e| YsError::external_error(e))?;
            let id_val = row.get_value(1).map_err(|e| YsError::external_error(e))?;

            if let (limbo::Value::Text(name), limbo::Value::Blob(bytes)) = (name_val, id_val) {
                let id = ObjectID::from_bytes(&bytes)?;
                branches.push((name, id));
            }
        }
        Ok(branches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_limbo_fs_storage() {
        let dir = tempdir().unwrap();
        let storage = LimboFsStorage::new(dir.path().to_path_buf()).await.unwrap();

        // Test branch operations
        storage.set_branch_name("dev").await.unwrap();
        assert_eq!(storage.get_branch_name().await.unwrap(), "dev");

        let id = "test content".object_id();
        storage.set_branch_id("dev", id).await.unwrap();
        assert_eq!(storage.get_branch_id("dev").await.unwrap(), id);
        assert!(storage.branch_exists("dev").await.unwrap());

        let branches = storage.list_branches().await.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].0, "dev");
        assert_eq!(branches[0].1, id);

        // Test object operations
        let text_file = storage.put_string("hello world").await.unwrap();
        assert!(storage.has(text_file.file_id).await.unwrap());
        assert_eq!(storage.get_string(text_file).await.unwrap(), "hello world");
    }
}
