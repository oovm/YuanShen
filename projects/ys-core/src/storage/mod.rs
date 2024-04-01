mod in_memory;
use crate::{
    objects::{object_store::YuanShenObject, text_file::TextFile},
    ObjectID, YsError, YsErrorKind,
};
pub use in_memory::MemoryObjectPool;
use std::future::Future;
use tokio::{fs::File, io::AsyncWriteExt};

/// An object proxy that specifies various capabilities
pub trait YuanShenClient {
    /// Check if a given object exists
    fn has(&self, id: ObjectID) -> impl Future<Output = Result<bool, YsError>> + Send;

    /// Try to get the string in TextFile
    fn get_string(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;

    fn get_string_file(&self, text: TextFile, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;

    fn put_string(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;

    fn put_string_file(&self, file: &mut File) -> impl Future<Output = Result<TextFile, YsError>> + Send;
}
