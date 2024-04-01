use super::*;



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

    async fn get_string_file(&self, text: TextFile, file: &mut File) -> Result<(), YsError> {
        let string = self.get_string(text).await?;
        Ok(file.write_all(string.as_bytes()).await?)
    }

    async fn put_string(&self, text: &str) -> Result<TextFile, YsError> {
        let id = text.as_bytes().object_id();
        self.objects.insert(id, text.as_bytes().to_vec());
        Ok(TextFile { file_id: id })
    }

    async fn put_string_file(&self, file: &mut File) -> Result<TextFile, YsError> {
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer).await?;
        let id = buffer.as_bytes().object_id();
        self.objects.insert(id, buffer.as_bytes().to_vec());
        Ok(TextFile { file_id: id })
    }

    async fn get_buffer(&self, text: TextFile) -> Result<String, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, text: TextFile, file: &mut File) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, text: &str) -> Result<TextFile, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, file: &mut File) -> Result<TextFile, YsError> {
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
