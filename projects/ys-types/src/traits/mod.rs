use crate::{
    objects::{ObjectID, TextFile},
    YsError,
};
use std::future::Future;
use std::path::Path;
use tokio::fs::File;

pub trait YuanShenID {
    type Object: YuanShenObject;

    fn load<O>(&self, store: &O) -> impl Future<Output = Result<Self::Object, YsError>>
    where
        O: ObjectProxy + Send + Sync;
}

pub trait YuanShenObject {
    fn object_id(&self) -> ObjectID;
}

/// An object proxy that specifies various capabilities
pub trait ObjectProxy {
    fn has(&self, id: ObjectID) -> impl Future<Output = Result<bool, YsError>> + Send;
    fn get_string(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;
    fn get_string_file(&self, text: TextFile, file: &Path) -> impl Future<Output = Result<(), YsError>> + Send;
    fn put_string(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;
    fn put_string_file(&self, file: &Path) -> impl Future<Output = Result<TextFile, YsError>> + Send;
    fn get_buffer(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;
    fn get_buffer_file(&self, text: TextFile, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;
    fn put_buffer(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;
    fn put_buffer_file(&self, file: &mut File) -> impl Future<Output = Result<TextFile, YsError>> + Send;

    // Added methods for typed access
    fn get_typed<T: for<'de> serde::Deserialize<'de> + Send>(&self, id: ObjectID) -> impl Future<Output = Result<T, YsError>> + Send;
    fn put_typed<T: serde::Serialize + YuanShenObject + Send + Sync>(&self, obj: &T) -> impl Future<Output = Result<ObjectID, YsError>> + Send;
}

pub trait ObjectStore: ObjectProxy {}
impl<T: ObjectProxy> ObjectStore for T {}

pub trait BranchProxy {
    fn current(&self) -> impl Future<Output = Result<String, YsError>> + Send;

    fn has_branch(&self, branch: &str) -> impl Future<Output = Result<bool, YsError>> + Send;

    fn get_branch(&self, branch: &str) -> impl Future<Output = Result<ObjectID, YsError>> + Send;
    fn set_branch(&self, branch: &str) -> impl Future<Output = Result<(), YsError>> + Send;
}
