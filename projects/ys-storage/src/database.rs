use ys_types::{
    objects::{ObjectID, TextFile},
    traits::{ObjectProxy, BranchProxy},
    YsError, YuanShenObject,
};
use std::path::Path;
use tokio::fs::File;

/// 基于 Limbo 的对象存储实现
#[derive(Debug)]
pub struct DatabaseObjectStore {
    database: limbo::Database,
}

impl DatabaseObjectStore {
    pub fn new(database: limbo::Database) -> Self {
        Self { database }
    }

    /// 初始化数据库表
    pub async fn initialize(&self) -> Result<(), YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        
        connection.execute(
            "CREATE TABLE IF NOT EXISTS objects (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            (),
        )
        .map_err(|e| YsError::external_error(e))?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS branches (
                name TEXT PRIMARY KEY,
                tree_id TEXT NOT NULL
            )",
            (),
        )
        .map_err(|e| YsError::external_error(e))?;

        Ok(())
    }

    /// 获取所有分支及其对应的 Tree ID
    pub async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        let mut statement = connection.prepare("SELECT name, tree_id FROM branches")
            .map_err(|e| YsError::external_error(e))?;
        
        let mut rows = statement.query(()).map_err(|e| YsError::external_error(e))?;
        
        let mut branches = Vec::new();
        while let Some(row) = rows.next().map_err(|e| YsError::external_error(e))? {
            let name = row.get::<String>(0).map_err(|e| YsError::external_error(e))?;
            let tree_id_str = row.get::<String>(1).map_err(|e| YsError::external_error(e))?;
            let tree_id = ObjectID::from_bytes(tree_id_str.as_bytes())?;
            branches.push((name, tree_id));
        }
        Ok(branches)
    }
}

#[async_trait::async_trait]
impl ObjectProxy for DatabaseObjectStore {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        let mut statement = connection.prepare("SELECT 1 FROM objects WHERE id = ?")
            .map_err(|e| YsError::external_error(e))?;
        let mut rows = statement.query((id.hash256.to_string(),))
            .map_err(|e| YsError::external_error(e))?;
        
        Ok(rows.next().map_err(|e| YsError::external_error(e))?.is_some())
    }

    async fn get_string(&self, id: TextFile) -> Result<String, YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        let mut statement = connection.prepare("SELECT data FROM objects WHERE id = ?")
            .map_err(|e| YsError::external_error(e))?;
        let mut rows = statement.query((id.file_id.hash256.to_string(),))
            .map_err(|e| YsError::external_error(e))?;
        
        let row = rows.next().map_err(|e| YsError::external_error(e))?
            .ok_or_else(|| YsError::invalid_object("Object not found"))?;
        
        let data_json: serde_json::Value = serde_json::from_str(&row.get::<String>(0).unwrap_or_default())
            .map_err(|e| YsError::external_error(e))?;
        
        Ok(data_json["content"].as_str().unwrap_or_default().to_string())
    }

    async fn get_string_file(&self, id: TextFile, path: &Path) -> Result<(), YsError> {
        let content = self.get_string(id).await?;
        tokio::fs::write(path, content).await.map_err(|e| YsError::external_error(e))?;
        Ok(())
    }

    async fn put_string(&self, _text: &str) -> Result<TextFile, YsError> {
        todo!("Implement put_string with ID calculation")
    }

    async fn put_string_file(&self, path: &Path) -> Result<TextFile, YsError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| YsError::external_error(e))?;
        self.put_string(&content).await
    }

    async fn get_buffer(&self, _id: TextFile) -> Result<String, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, _id: TextFile, _file: &mut File) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, _text: &str) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, _file: &mut File) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn get_typed<T: for<'de> serde::Deserialize<'de> + Send>(&self, id: ObjectID) -> Result<T, YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        let mut statement = connection.prepare("SELECT data FROM objects WHERE id = ?")
            .map_err(|e| YsError::external_error(e))?;
        let mut rows = statement.query((id.hash256.to_string(),))
            .map_err(|e| YsError::external_error(e))?;
        
        let row = rows.next().map_err(|e| YsError::external_error(e))?
            .ok_or_else(|| YsError::invalid_object("Object not found"))?;
        
        Ok(serde_json::from_str(&row.get::<String>(0).unwrap_or_default())?)
    }

    async fn put_typed<T: serde::Serialize + YuanShenObject + Send + Sync>(&self, obj: &T) -> Result<ObjectID, YsError> {
        let id = obj.object_id();
        let data = serde_json::to_string(obj).map_err(|e| YsError::external_error(e))?;
        
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        connection.execute(
            "INSERT INTO objects (id, data) VALUES (?, ?) ON CONFLICT (id) DO UPDATE SET data = excluded.data",
            (id.hash256.to_string(), data),
        )
        .map_err(|e| YsError::external_error(e))?;
        
        Ok(id)
    }
}

#[async_trait::async_trait]
impl BranchProxy for DatabaseObjectStore {
    async fn current(&self) -> Result<String, YsError> {
        Ok("main".to_string())
    }

    async fn has_branch(&self, branch: &str) -> Result<bool, YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        let mut statement = connection.prepare("SELECT 1 FROM branches WHERE name = ?")
            .map_err(|e| YsError::external_error(e))?;
        let mut rows = statement.query((branch,))
            .map_err(|e| YsError::external_error(e))?;
        
        Ok(rows.next().map_err(|e| YsError::external_error(e))?.is_some())
    }

    async fn get_branch(&self, branch: &str) -> Result<ObjectID, YsError> {
        let connection = self.database.connect().map_err(|e| YsError::external_error(e))?;
        let mut statement = connection.prepare("SELECT tree_id FROM branches WHERE name = ?")
            .map_err(|e| YsError::external_error(e))?;
        let mut rows = statement.query((branch,))
            .map_err(|e| YsError::external_error(e))?;
        
        let row = rows.next().map_err(|e| YsError::external_error(e))?
            .ok_or_else(|| YsError::invalid_object("Branch not found"))?;
        
        let tree_id_str = row.get::<String>(0).map_err(|e| YsError::external_error(e))?;
        ObjectID::from_bytes(tree_id_str.as_bytes())
    }

    async fn set_branch(&self, branch: &str) -> Result<(), YsError> {
        // This would need a tree_id, but the trait only takes branch name
        // For now, let's assume it's for switching or we need to extend the trait
        todo!("set_branch needs tree_id")
    }

    async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError> {
        self.list_branches().await
    }
}
