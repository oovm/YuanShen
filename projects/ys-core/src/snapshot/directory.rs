use std::{
    collections::BTreeMap,
    fs::{read_dir, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{IgnoreRules, ObjectID, ObjectStore, YsError};


/// A directory tree, with [`ObjectID`]s at the leaves.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapShotDirectory {
    #[serde(flatten)]
    pub root: BTreeMap<String, DirectoryEntry>,
}



impl SnapShotDirectory {
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

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum DirectoryEntry {
    Directory(Box<SnapShotDirectory>),
    File(ObjectID),
}

impl SnapShotDirectory {
    pub fn new<Store: ObjectStore>(dir: &Path, ignores: &IgnoreRules, store: &mut Store) -> Result<Box<Self>, YsError> {
        let mut root = BTreeMap::new();
        for f in std::fs::read_dir(dir)? {
            let dir_entry = f?;
            if ignores.glob.contains(&dir_entry.file_name().into_string().unwrap()) {
                continue;
            }
            let file_type = dir_entry.file_type()?;
            if file_type.is_dir() {
                let directory = SnapShotDirectory::new(dir_entry.path().as_path(), ignores, store)?;
                root.insert(dir_entry.file_name().into_string().unwrap(), DirectoryEntry::Directory(directory));
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
        Ok(Box::new(SnapShotDirectory { root }))
    }
}
