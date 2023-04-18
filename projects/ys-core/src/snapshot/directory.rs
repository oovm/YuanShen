use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fs::{read_dir, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{ObjectID, ObjectStore};

/// A directory tree, with [`ObjectID`]s at the leaves.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapShotDirectory {
    #[serde(flatten)]
    pub root: BTreeMap<String, DirectoryEntry>,
}

#[derive(Debug)]
pub enum Error<Store: ObjectStore> {
    ObjectMissing(ObjectID),
    Store(Store::Error),
    IO(std::io::Error),
}

impl SnapShotDirectory {
    /// Write out the directory structure at the given directory path.
    ///
    /// The target directory must already exist.
    pub async fn write<Store: ObjectStore>(&self, store: &Store, path: &Path) -> Result<(), Error<Store>> {
        if read_dir(path).is_ok() {
            for (file_name, entry) in self.root.iter() {
                match entry {
                    DirectoryEntry::File(id) => {
                        let v = store.read(*id).await.map_err(Error::Store)?;
                        let mut f = File::options().create(true).write(true).open(path.join(file_name)).map_err(Error::IO)?;
                        f.write(&v).map_err(Error::IO)?;
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
    pub fn new<Store: ObjectStore>(dir: &Path, ignores: &IgnoreRules, store: &mut Store) -> Result<Box<Self>, Error<Store>> {
        let mut root = BTreeMap::new();
        for f in std::fs::read_dir(dir).map_err(Error::IO)? {
            let dir_entry = f.map_err(Error::IO)?;
            if ignores.glob.contains(&dir_entry.file_name().into_string().unwrap()) {
                continue;
            }
            let file_type = dir_entry.file_type().map_err(Error::IO)?;
            if file_type.is_dir() {
                let directory = SnapShotDirectory::new(dir_entry.path().as_path(), ignores, store)?;
                root.insert(dir_entry.file_name().into_string().unwrap(), DirectoryEntry::Directory(directory));
            }
            else if file_type.is_file() {
                let id = ObjectID::try_from(dir_entry.path().as_path()).map_err(Error::IO)?;
                root.insert(dir_entry.file_name().into_string().unwrap(), DirectoryEntry::File(id));
                let mut v = Vec::new();
                let mut obj_file = File::options().read(true).open(dir_entry.path()).map_err(Error::IO)?;
                obj_file.read_to_end(&mut v).map_err(Error::IO)?;
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
