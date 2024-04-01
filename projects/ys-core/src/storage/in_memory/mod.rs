use super::*;
use crate::{objects::IncrementalTextFile, utils::{from_json}, YuanShenObject};
use std::{future::Future, pin::Pin};

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



impl YuanShenClient for MemoryObjectPool {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        Ok(self.objects.contains_key(&id))
    }

    fn get_string<'a>(&'a self, text: StandaloneText) -> Pin<Box<dyn Future<Output = Result<String, YsError>> + Send + 'a>> {
        Box::pin(async move {
            let data = self.get_text_data(text.file_id)?;
            data.resolve(self).await
        })
    }

    async fn get_string_file(&self, text: StandaloneText, mut file: File) -> Result<File, YsError> {
        let string = self.get_string(text).await?;
        file.write_all(string.as_bytes()).await?;
        Ok(file)
    }

    async fn put_string(&self, text: &str) -> Result<StandaloneText, YsError> {
        let id = text.as_bytes().object_id();
        self.objects.insert(id, text.as_bytes().to_vec());
        Ok(StandaloneText { file_id: id })
    }

    async fn put_string_file(&self, file: &mut File) -> Result<StandaloneText, YsError> {
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer).await?;
        let id = buffer.as_bytes().object_id();
        self.objects.insert(id, buffer.as_bytes().to_vec());
        Ok(StandaloneText { file_id: id })
    }

    async fn get_buffer(&self, text: StandaloneText) -> Result<String, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, text: StandaloneText, file: &mut File) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, text: &str) -> Result<StandaloneText, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, file: &mut File) -> Result<StandaloneText, YsError> {
        todo!()
    }
}

impl MemoryObjectPool {
    fn get_text_data(&self, id: ObjectID) -> Result<IncrementalTextFile, YsError> {
        Ok(match self.objects.get(&id) {
            Some(o) => from_json(o.as_slice())?,
            None => Err(YsErrorKind::MissingObject { id })?,
        })
    }
}