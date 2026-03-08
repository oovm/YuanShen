use std::path::Path;
use ys_types::{ObjectID, YsError};
use ys_storage::StorageBackend;
use crate::Driver;

pub struct P4Driver<S> {
    store: S,
}

impl<S> P4Driver<S> 
where 
    S: StorageBackend + 'static
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> Driver for P4Driver<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "p4"
    }

    async fn clone(&self, url: &str, _local_path: &Path) -> Result<(), YsError> {
        println!("Cloning P4 repository from {} into database", url);
        Err(YsError::not_implemented("P4Driver::clone_to_db"))
    }

    async fn fetch(&self, url: &str) -> Result<ObjectID, YsError> {
        println!("Fetching P4 repository from {}", url);
        Err(YsError::not_implemented("P4Driver::fetch_to_db"))
    }

    async fn push(&self, url: &str, commit_id: ObjectID) -> Result<(), YsError> {
        println!("Pushing commit {} to P4 repository {}", commit_id, url);
        Err(YsError::not_implemented("P4Driver::push_from_db"))
    }
}
