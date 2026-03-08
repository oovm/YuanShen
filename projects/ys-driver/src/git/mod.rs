use std::path::Path;
use ys_types::{ObjectID, YsError};
use ys_storage::StorageBackend;
use crate::Driver;

pub struct GitDriver<S> {
    store: S,
}

impl<S> GitDriver<S> 
where 
    S: StorageBackend + 'static
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> Driver for GitDriver<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "git"
    }

    async fn clone(&self, url: &str, _local_path: &Path) -> Result<(), YsError> {
        println!("Cloning Git repository from {} into database", url);
        Err(YsError::not_implemented("GitDriver::clone_to_db"))
    }

    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        println!("Fetching Git repository from {}", url);
        Err(YsError::not_implemented("GitDriver::fetch_to_db"))
    }

    async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError> {
        println!("Pushing Git commit {} to {}", commit_id, url);
        Err(YsError::not_implemented("GitDriver::push_from_db"))
    }
}
