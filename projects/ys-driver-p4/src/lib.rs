use std::path::Path;
use ys_types::{ObjectID, YsError};
use ys_driver::Driver;

pub struct P4Driver;

impl P4Driver {
    pub fn new() -> Self {
        Self
    }
}

impl Driver for P4Driver {
    fn name(&self) -> &'static str {
        "p4"
    }

    async fn clone(&self, _url: &str, _local_path: &Path) -> Result<(), YsError> {
        Err(YsError::not_implemented("P4Driver::clone"))
    }

    async fn fetch(&self, _url: &str) -> Result<ObjectID, YsError> {
        Err(YsError::not_implemented("P4Driver::fetch"))
    }

    async fn push(&self, _url: &str, _commit_id: ObjectID) -> Result<(), YsError> {
        Err(YsError::not_implemented("P4Driver::push"))
    }
}
