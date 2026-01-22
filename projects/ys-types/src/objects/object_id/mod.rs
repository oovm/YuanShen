use crate::utils::{read_json, write_json};
use crate::errors::YsError;
use uuid::Uuid;
use std::{
    fmt::{Display, Debug},
    io,
    path::Path,
};
use serde::{Deserialize, Serialize};

mod convert;
mod hasher;

pub use hasher::ObjectHasher;

/// ObjectID represents a unique identifier for an object (UUID based)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectID(pub Uuid);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchJson {
    pub tree_id: Uuid,
}

impl ObjectID {
    pub fn new() -> Self {
        ObjectID(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        ObjectID(uuid)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, YsError> {
        let uuid = Uuid::from_slice(bytes).map_err(|e| YsError::external_error(io::Error::new(io::ErrorKind::InvalidData, e.to_string())))?;
        Ok(ObjectID(uuid))
    }

    pub fn read_branch(dot_ys: &Path, name: &str) -> Result<Self, YsError> {
        let file = dot_ys.join("branches").join(name);
        let json = read_json::<BranchJson>(&file)?;
        Ok(ObjectID(json.tree_id))
    }

    pub fn write_branch(&self, dot_ys: &Path, name: &str) -> Result<(), YsError> {
        let file = dot_ys.join("branches").join(name);
        let json = BranchJson { tree_id: self.0 };
        write_json(&json, &file)
    }
}

impl Display for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectID({})", self.0)
    }
}

impl From<Uuid> for ObjectID {
    fn from(uuid: Uuid) -> Self {
        ObjectID(uuid)
    }
}
