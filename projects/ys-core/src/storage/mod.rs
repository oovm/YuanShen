use crate::{
    objects::{object_store::YuanShenObject, text_file::TextFile},
    ObjectID, YsError, YsErrorKind,
};
use std::future::Future;
use tokio::{fs::File, io::AsyncWriteExt};

pub trait ObjectProxy {
    /// Check if a given object exists
    fn has(&self, id: ObjectID) -> impl Future<Output = Result<bool, YsError>> + Send;

    fn get_string(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;

    fn get_string_file(&self, text: TextFile, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;

    fn put_string(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;
}

pub struct FakeObjectManager {}

impl FakeObjectManager {}

impl ObjectProxy for FakeObjectManager {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        Ok(id.hash256.as_bytes().ends_with(&[0]))
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
