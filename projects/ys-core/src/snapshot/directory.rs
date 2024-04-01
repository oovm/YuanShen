use std::{
    collections::BTreeMap,
    fs::{read_dir, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

use crate::{objects::object_store::YuanShenObject, IgnoreRules, ObjectID, ObjectStore, YsError};

// A directory tree, with [`ObjectID`]s at the leaves.
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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub enum DirectoryEntry {
    Directory(DirectoryObject),
    File(FileObject),
    /// A reference to other snapshots.
    Subtree(SubTreeObject),
}

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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        todo!()
    }
}

pub struct FileObject {
    id: ObjectID,
}

pub struct SubTreeObject {
    id: ObjectID,
}

impl YuanShenObject for SnapShotTree {}

impl SnapShotTree {
    /// Write out the directory structure at the given directory path.
    ///
    /// The target directory must already exist.
    pub async fn write<Store: ObjectStore>(&self, store: &Store, path: &Path) -> Result<(), YsError> {
        if read_dir(path).is_ok() {
            for (file_name, entry) in self.root.iter() {
                match entry {
                    DirectoryEntry::File(id) => {
                        let v = store.get(*id).await?;
                        let mut f = File::options().create(true).write(true).open(path.join(file_name))?;
                        f.write(&v)?;
                    }
                    DirectoryEntry::Directory(dir) => {
                        dir.write(store, PathBuf::from(path).join(file_name).as_path()).await?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl SnapShotTree {
    pub fn new<Store: ObjectStore>(dir: &Path, ignores: &IgnoreRules, store: &mut Store) -> Result<Self, YsError> {
        let mut root = BTreeMap::new();
        for f in std::fs::read_dir(dir)? {
            let dir_entry = f?;
            if ignores.glob.contains(&dir_entry.file_name().into_string().unwrap()) {
                continue;
            }
            let file_type = dir_entry.file_type()?;
            if file_type.is_dir() {
                let directory = SnapShotTree::new(dir_entry.path().as_path(), ignores, store)?;
                root.insert(dir_entry.file_name().into_string().unwrap(), DirectoryEntry::Directory(Box::new(directory)));
            }
            else if file_type.is_file() {
                let id = ObjectID::try_from(dir_entry.path().as_path())?;
                root.insert(dir_entry.file_name().into_string().unwrap(), DirectoryEntry::File(id));
                let mut v = Vec::new();
                let mut obj_file = File::options().read(true).open(dir_entry.path())?;
                obj_file.read_to_end(&mut v)?;
                todo!()
                // store.insert(&v).await.map_err(Error::Store)?;
            }
            else {
                eprintln!("TODO support things which aren't files or directories: {:?}", dir_entry.file_name());
            }
        }
        Ok(SnapShotTree { root })
    }
}
