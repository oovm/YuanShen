use crate::{
    objects::{ObjectID, TextFile},
    YsError,
};
use std::path::Path;
use tokio::fs::File;

pub trait YuanShenID {
    type Object: YuanShenObject;

    fn load<O>(&self, store: &O) -> impl std::future::Future<Output = Result<Self::Object, YsError>>
    where
        O: ObjectProxy + Send + Sync;
}

pub trait YuanShenObject {
    fn object_id(&self) -> ObjectID;
}

/// An object proxy that specifies various capabilities
pub trait ObjectProxy: Send + Sync {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError>;
    async fn get_string(&self, text: TextFile) -> Result<String, YsError>;
    async fn get_string_file(&self, text: TextFile, file: &Path) -> Result<(), YsError>;
    async fn put_string(&self, text: &str) -> Result<TextFile, YsError>;
    async fn put_string_file(&self, file: &Path) -> Result<TextFile, YsError>;
    async fn get_buffer(&self, text: TextFile) -> Result<Vec<u8>, YsError>;
    async fn get_buffer_file(&self, text: TextFile, file: &Path) -> Result<(), YsError>;
    async fn put_buffer(&self, buf: &[u8]) -> Result<TextFile, YsError>;
    async fn put_buffer_file(&self, file: &Path) -> Result<TextFile, YsError>;

    // Added methods for typed access
    async fn get_typed<T>(&self, id: ObjectID) -> Result<T, YsError>
    where
        T: for<'de> serde::Deserialize<'de> + Send;

    async fn put_typed<T>(&self, obj: &T) -> Result<ObjectID, YsError>
    where
        T: serde::Serialize + YuanShenObject + Send + Sync;
}

pub trait ObjectStore: ObjectProxy {}
impl<T: ObjectProxy> ObjectStore for T {}

pub trait BranchProxy: Send + Sync {
    /// Get the name of the current branch
    async fn get_branch_name(&self) -> Result<String, YsError>;

    /// Set the current branch name
    async fn set_branch_name(&self, name: &str) -> Result<(), YsError>;

    /// Get the tip (ObjectID) of a branch
    async fn get_branch_id(&self, name: &str) -> Result<ObjectID, YsError>;

    /// Set the tip (ObjectID) of a branch
    async fn set_branch_id(&self, name: &str, id: ObjectID) -> Result<(), YsError>;

    /// Check if a branch exists
    async fn branch_exists(&self, name: &str) -> Result<bool, YsError>;

    /// List all branches and their tips
    async fn list_branches(&self) -> Result<Vec<(String, ObjectID)>, YsError>;
}
