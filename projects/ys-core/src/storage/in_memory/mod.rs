use super::*;
use crate::{
    objects::{ObjectID, StandaloneTextFile},
    YuanShenClient,
};
use dashmap::DashMap;
use tokio::io::AsyncReadExt;

/// [YuanShenClient] in memory, all changes will disappear after the program exits, used for testing.
#[derive(Clone, Debug)]
pub struct MemoryObjectPool {
    objects: DashMap<ObjectID, Vec<u8>>,
}

impl Default for MemoryObjectPool {
    fn default() -> Self {
        Self { objects: Default::default() }
    }
}

impl MemoryObjectPool {}

impl YuanShenClient for MemoryObjectPool {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        Ok(self.objects.contains_key(&id))
    }

    async fn get_string(&self, text: StandaloneTextFile) -> Result<String, YsError> {
        match self.objects.get(&text.string_id) {
            Some(o) => Ok(String::from_utf8(o.to_vec())?),
            None => Err(YsErrorKind::MissingObject { id: text.string_id })?,
        }
    }

    async fn get_string_file(&self, text: StandaloneTextFile, file: &mut File) -> Result<(), YsError> {
        let string = self.get_string(text).await?;
        Ok(file.write_all(string.as_bytes()).await?)
    }

    async fn put_string(&self, text: &str) -> Result<StandaloneTextFile, YsError> {
        let id = ObjectID { hash256: blake3::hash(text.as_bytes()) };
        self.objects.insert(id, text.as_bytes().to_vec());
        Ok(StandaloneTextFile { string_id: id })
    }

    async fn put_string_file(&self, file: &mut File) -> Result<StandaloneTextFile, YsError> {
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer).await?;
        let id = ObjectID { hash256: blake3::hash(buffer.as_bytes()) };
        self.objects.insert(id, buffer.as_bytes().to_vec());
        Ok(StandaloneTextFile { string_id: id })
    }

    async fn get_buffer(&self, text: StandaloneTextFile) -> Result<String, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, text: StandaloneTextFile, file: &mut File) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, text: &str) -> Result<StandaloneTextFile, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, file: &mut File) -> Result<StandaloneTextFile, YsError> {
        todo!()
    }
}
