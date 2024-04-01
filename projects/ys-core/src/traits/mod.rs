use crate::{
    objects::{ObjectID, TextFile},
    YsError,
};
use std::future::Future;
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
    /// Check if a given object exists
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, ObjectProxy, YuanShenObject, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// let invalid = ".ys".object_id();
    /// assert!(client.has(id.file_id).await?);
    /// assert!(!client.has(invalid).await?);
    /// # Ok(())
    /// # })
    /// ```
    fn has(&self, id: ObjectID) -> impl Future<Output = Result<bool, YsError>> + Send;

    /// Try to get the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, ObjectProxy, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn get_string(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;

    /// Try to get the string in TextFile
    fn get_string_file(&self, text: TextFile, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;

    /// Try to put the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, ObjectProxy, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn put_string(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;

    /// Try to put the string in TextFile
    fn put_string_file(&self, file: &mut File) -> impl Future<Output = Result<TextFile, YsError>> + Send;

    /// Try to get the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, ObjectProxy, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn get_buffer(&self, text: TextFile) -> impl Future<Output = Result<String, YsError>> + Send;

    /// Try to get the string in TextFile
    fn get_buffer_file(&self, text: TextFile, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;

    /// Try to put the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, ObjectProxy, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn put_buffer(&self, text: &str) -> impl Future<Output = Result<TextFile, YsError>> + Send;

    /// Try to put the string in TextFile
    fn put_buffer_file(&self, file: &mut File) -> impl Future<Output = Result<TextFile, YsError>> + Send;
}

pub trait BranchProxy {
    fn current(&self) -> impl Future<Output = Result<String, YsError>> + Send;
    
    fn has_branch(&self, branch: &str) -> impl Future<Output = Result<bool, YsError>> + Send;
    
    fn get_branch(&self, branch: &str) -> impl Future<Output = Result<ObjectID, YsError>> + Send;
    fn set_branch(&self, branch: &str) -> impl Future<Output = Result<(), YsError>> + Send;
}
