use super::*;
use std::path::Path;

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
        self.get_text_data(text.file_id)?.resolve(self).await
    }

    async fn get_string_file(&self, text: TextFile, file: &Path) -> Result<(), YsError> {
        let string = self.get_string(text).await?;
        truncate_write(file.to_path_buf(), string.as_bytes()).await
    }

    async fn put_string(&self, text: &str) -> Result<TextFile, YsError> {
        let id = text.as_bytes().object_id();
        self.objects.insert(id, text.as_bytes().to_vec());
        Ok(TextFile { file_id: id })
    }

    async fn put_string_file(&self, file: &Path) -> Result<TextFile, YsError> {
        let buffer = read_to_string(file.to_path_buf()).await?;
        let id = buffer.object_id();
        self.objects.insert(id, buffer.as_bytes().to_vec());
        Ok(TextFile { file_id: id })
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

    async fn put_buffer_file(&self, _: &mut File) -> Result<TextFile, YsError> {
        todo!()
    }
}

impl MemoryObjectPool {
    fn get_text_data(&self, id: ObjectID) -> Result<TextIncrementalData, YsError> {
        Ok(match self.objects.get(&id) {
            Some(o) => from_json(o.as_slice())?,
            None => Err(YsErrorKind::MissingObject { id })?,
        })
    }
}
