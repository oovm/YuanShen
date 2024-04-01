use crate::{
    objects::{ObjectID, StandaloneText},
    YsError,
};
use std::{future::Future, pin::Pin};
use tokio::fs::File;

pub trait YuanShenObject {
    fn object_id(&self) -> ObjectID;
}

/// An object proxy that specifies various capabilities
pub trait YuanShenClient {
    /// Check if a given object exists
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, YuanShenClient, YuanShenObject, storage::{MemoryObjectPool}};
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
    /// # use ys_core::{async_test, YuanShenClient, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn get_string<'a>(&'a self, text: StandaloneText) -> Pin<Box<dyn Future<Output = Result<String, YsError>> + Send + 'a>>;

    /// Try to get the string in TextFile
    fn get_string_file(&self, text: StandaloneText, file: File) -> impl Future<Output = Result<File, YsError>> + Send;

    /// Try to put the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, YuanShenClient, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn put_string(&self, text: &str) -> impl Future<Output = Result<StandaloneText, YsError>> + Send;

    /// Try to put the string in TextFile
    fn put_string_file(&self, file: &mut File) -> impl Future<Output = Result<StandaloneText, YsError>> + Send;

    /// Try to get the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, YuanShenClient, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn get_buffer(&self, text: StandaloneText) -> impl Future<Output = Result<String, YsError>> + Send;

    /// Try to get the string in TextFile
    fn get_buffer_file(&self, text: StandaloneText, file: &mut File) -> impl Future<Output = Result<(), YsError>> + Send;

    /// Try to put the string in TextFile
    ///
    /// ## Examples
    ///
    /// ```
    /// # use ys_core::{async_test, YuanShenClient, storage::{MemoryObjectPool}};
    /// # async_test(async {
    /// let client = MemoryObjectPool::default();
    /// let id = client.put_string("Wo chao! Yuan!").await?;
    /// assert_eq!(client.get_string(id).await?, "Wo chao! Yuan!");
    /// # Ok(())
    /// # })
    /// ```
    fn put_buffer(&self, text: &str) -> impl Future<Output = Result<StandaloneText, YsError>> + Send;

    /// Try to put the string in TextFile
    fn put_buffer_file(&self, file: &mut File) -> impl Future<Output = Result<StandaloneText, YsError>> + Send;
}
