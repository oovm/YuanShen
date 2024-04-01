mod fake;
use crate::{
    objects::{object_store::YuanShenObject, text_file::TextFile},
    ObjectID, YsError, YsErrorKind,
};
pub use fake::FakeObjectManager;
use std::future::Future;
use tokio::{fs::File, io::AsyncWriteExt};

pub trait ObjectProxy {
    /// Check if a given object exists
    fn has(&self, id: ObjectID) -> impl Future<Output = Result<bool, YsError>> + Send;

    fn get_string(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;

    fn get_string_file(&self, text: TextFile, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;

    fn put_string(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;
}
