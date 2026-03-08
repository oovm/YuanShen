use std::path::Path;
use ys_types::{ObjectID, YsError};
use ys_storage::StorageBackend;
use crate::Driver;

pub struct SvnDriver<S> {
    store: S,
}

impl<S> SvnDriver<S> 
where 
    S: StorageBackend + 'static
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> Driver for SvnDriver<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "svn"
    }

    async fn clone(&self, url: &str, _local_path: &Path) -> Result<(), YsError> {
        println!("Cloning SVN repository from {} into database", url);
        Err(YsError::not_implemented("SvnDriver::clone_to_db"))
    }

    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        println!("Fetching SVN repository from {}", url);
        Err(YsError::not_implemented("SvnDriver::fetch_to_db"))
    }

    async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError> {
        println!("Pushing commit {} to SVN repository {}", commit_id, url);
        Err(YsError::not_implemented("SvnDriver::push_from_db"))
    }
}
