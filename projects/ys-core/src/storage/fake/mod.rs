use super::*;

#[derive(Copy, Clone, Debug)]
pub struct FakeObjectManager {
    
}

impl FakeObjectManager {}

impl ObjectProxy for FakeObjectManager {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        let last = unsafe { id.hash256.as_bytes().last().unwrap_unchecked() };
        Ok(*last % 2 == 0)
    }

    async fn get_string(&self, text: TextFile) -> Result<String, YsError> {
        match self.has(text.string_id).await? {
            true => Ok("FakeObjectManager".to_string()),
            false => Err(YsErrorKind::MissingObject { id: text.string_id })?,
        }
    }

    async fn get_string_file(&self, text: TextFile, file: &mut File) -> Result<(), YsError> {
        let string = self.get_string(text).await?;
        Ok(file.write_all(string.as_bytes()).await?)
    }

    async fn put_string(&self, text: &str) -> Result<TextFile, YsError> {
        todo!()
    }
}
