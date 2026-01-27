use std::{collections::BTreeMap, path::Path};

use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    objects::{IgnoreRules, ObjectID, TextFile, },
    traits::YuanShenObject,
    YsError, ObjectProxy,
};
use crate::objects::{ TextIncrementalFile};

/// A directory tree, with [`ObjectID`]s at the leaves.
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct SnapShotTree {
    pub root: BTreeMap<String, DirectoryEntry>,
}

impl Serialize for SnapShotTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.root.len()))?;
        for (name, entry) in self.root.iter() {
            map.serialize_entry(name, entry)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for SnapShotTree {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectoryEntry {
    Directory(DirectoryObject),
    TextStandalone(TextFile),
    TextIncremental(TextIncrementalFile),
    /// A reference to other snapshots.
    Subtree(SubTreeObject),
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DirectoryObject {
    entries: BTreeMap<String, DirectoryEntry>,
}

impl Serialize for DirectoryObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.entries.len()))?;
        for (name, entry) in self.entries.iter() {
            map.serialize_entry(name, entry)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for DirectoryObject {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubTreeObject {
    id: ObjectID,
}

impl YuanShenObject for SnapShotTree {
    fn object_id(&self) -> ObjectID {
        todo!()
    }
}

impl SnapShotTree {
    /// Write out the directory structure at the given directory path.
    ///
    /// The target directory must already exist.
    pub async fn write<Store: ObjectProxy>(&self, _store: &Store, _path: &Path) -> Result<(), YsError> {
        todo!();
    }
}

impl SnapShotTree {
    pub fn new<Store: ObjectProxy>(_dir: &Path, _ignores: &IgnoreRules, _store: &mut Store) -> Result<Self, YsError> {
        todo!();
    }
}
