use ys_types::{
    objects::{ObjectID, TextFile},
    ObjectProxy, BranchProxy,
    YsError, YsErrorKind, YuanShenObject,
};
use std::path::Path;
use tokio::fs::File;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// [ObjectProxy] in memory, all changes will disappear after the program exits, used for testing.
#[derive(Clone, Debug)]
pub struct MemoryObjectPool {
    objects: DashMap<ObjectID, Vec<u8>>,
    branches: DashMap<String, ObjectID>,
    current_branch: Arc<RwLock<String>>,
}

impl Default for MemoryObjectPool {
    fn default() -> Self {
        Self { 
            objects: Default::default(),
            branches: Default::default(),
            current_branch: Arc::new(RwLock::new("main".to_string())),
        }
    }
}

impl ObjectProxy for MemoryObjectPool {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        Ok(self.objects.contains_key(&id))
    }

    async fn get_string(&self, text: TextFile) -> Result<String, YsError> {
        match self.objects.get(&text.file_id) {
            Some(o) => {
                Ok(String::from_utf8_lossy(o.as_slice()).to_string())
            },
            None => Err(YsErrorKind::MissingObject { id: text.file_id })?,
        }
    }

    async fn get_string_file(&self, text: TextFile, file: &Path) -> Result<(), YsError> {
        let string = self.get_string(text).await?;
        tokio::fs::write(file, string).await.map_err(|e| YsError::external_error(e))?;
        Ok(())
    }

    async fn put_string(&self, text: &str) -> Result<TextFile, YsError> {
        let id = text.object_id();
        self.objects.insert(id, text.as_bytes().to_vec());
        Ok(TextFile { file_id: id })
    }

    async fn put_string_file(&self, file: &Path) -> Result<TextFile, YsError> {
        let content = tokio::fs::read_to_string(file).await.map_err(|e| YsError::external_error(e))?;
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
        match self.objects.get(&id) {
            Some(o) => Ok(serde_json::from_slice(o.as_slice())?),
            None => Err(YsErrorKind::MissingObject { id })?,
        }
    }

    async fn put_typed<T: serde::Serialize + YuanShenObject + Send + Sync>(&self, obj: &T) -> Result<ObjectID, YsError> {
        let id = obj.object_id();
        let bytes = serde_json::to_vec_pretty(obj)?;
        self.objects.insert(id, bytes);
        Ok(id)
    }
}

impl BranchProxy for MemoryObjectPool {
    async fn get_branch_name(&self) -> Result<String, YsError> {
        Ok(self.current_branch.read().await.clone())
    }

    async fn set_branch_name(&self, name: &str) -> Result<(), YsError> {
        let mut branch = self.current_branch.write().await;
        *branch = name.to_string();
        Ok(())
    }

    async fn get_branch_id(&self, name: &str) -> Result<ObjectID, YsError> {
        match self.branches.get(name) {
            Some(id) => Ok(*id),
            None => Err(YsError::invalid_object(format!("Branch not found: {}", name)))?,
        }
    }

    async fn set_branch_id(&self, name: &str, id: ObjectID) -> Result<(), YsError> {
        self.branches.insert(name.to_string(), id);
        Ok(())
    }

    async fn branch_exists(&self, name: &str) -> Result<bool, YsError> {
        Ok(self.branches.contains_key(name))
    }

    async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError> {
        Ok(self.branches.iter().map(|r| (r.key().clone(), *r.value())).collect())
    }
}
