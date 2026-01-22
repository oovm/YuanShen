use ys_types::{
    objects::{ObjectID, TextFile},
    ObjectProxy,
    YsError, YsErrorKind, YuanShenObject,
};
use std::path::Path;
use tokio::fs::File;
use dashmap::DashMap;

/// [ObjectProxy] in memory, all changes will disappear after the program exits, used for testing.
#[derive(Clone, Debug)]
pub struct MemoryObjectPool {
    objects: DashMap<ObjectID, Vec<u8>>,
}

impl Default for MemoryObjectPool {
    fn default() -> Self {
        Self { objects: Default::default() }
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
