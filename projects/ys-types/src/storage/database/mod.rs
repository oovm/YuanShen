use crate::{
    objects::{ObjectID, TextFile, TextIncrementalData},
    traits::ObjectProxy,
    YsError, YuanShenObject,
};
use sqlx::{Pool, Postgres};
use std::path::Path;
use tokio::fs::File;

/// 基于 PostgreSQL 的对象存储实现
#[derive(Debug)]
pub struct DatabaseObjectStore {
    pool: Pool<Postgres>,
}

impl DatabaseObjectStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// 初始化数据库表
    pub async fn init(&self) -> Result<(), YsError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS objects (
                id BYTEA PRIMARY KEY,
                data JSONB NOT NULL
            )"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| YsError::external_error(e))?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS branches (
                name TEXT PRIMARY KEY,
                tree_id BYTEA NOT NULL
            )"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| YsError::external_error(e))?;

        Ok(())
    }

    /// 获取所有分支及其对应的 Tree ID
    pub async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError> {
        let rows: Vec<(String, Vec<u8>)> = sqlx::query_as("SELECT name, tree_id FROM branches")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| YsError::external_error(e))?;
        
        let mut branches = Vec::new();
        for (name, tree_id_bytes) in rows {
            let tree_id = ObjectID::from_bytes(&tree_id_bytes)?;
            branches.push((name, tree_id));
        }
        Ok(branches)
    }
}

impl ObjectProxy for DatabaseObjectStore {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        let row = sqlx::query("SELECT 1 FROM objects WHERE id = $1")
            .bind(id.hash256.as_bytes())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| YsError::external_error(e))?;
        Ok(row.is_some())
    }

    async fn get_string(&self, id: TextFile) -> Result<String, YsError> {
        let row: (String,) = sqlx::query_as("SELECT data->>'content' FROM objects WHERE id = $1")
            .bind(id.file_id.hash256.as_bytes())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| YsError::external_error(e))?;
        Ok(row.0)
    }

    async fn get_string_file(&self, id: TextFile, path: &Path) -> Result<(), YsError> {
        let content = self.get_string(id).await?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    async fn put_string(&self, text: &str) -> Result<TextFile, YsError> {
        // 简化的实现，实际需要计算 ObjectID
        todo!("Implement put_string with ID calculation")
    }

    async fn put_string_file(&self, path: &Path) -> Result<TextFile, YsError> {
        let content = tokio::fs::read_to_string(path).await?;
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

    async fn get_typed<T: for<'de> serde::Deserialize<'de>>(&self, id: ObjectID) -> Result<T, YsError> {
        let row: (serde_json::Value,) = sqlx::query_as("SELECT data FROM objects WHERE id = $1")
            .bind(id.hash256.as_bytes())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| YsError::external_error(e))?;
        Ok(serde_json::from_value(row.0)?)
    }

    async fn put_typed<T: serde::Serialize + YuanShenObject>(&self, obj: &T) -> Result<ObjectID, YsError> {
        let id = obj.object_id();
        let data = serde_json::to_value(obj)?;
        sqlx::query("INSERT INTO objects (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(id.hash256.as_bytes())
            .bind(data)
            .execute(&self.pool)
            .await
            .map_err(|e| YsError::external_error(e))?;
        Ok(id)
    }
}
