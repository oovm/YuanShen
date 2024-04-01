use crate::{
    errors::YsError,  snapshot::directory::SnapShotTree,  DirectoryEntry,
      DOT_YUAN_SHEN,
};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    fs::{create_dir, create_dir_all, read_dir, read_to_string, try_exists, File},
    hash::Hash,
    io::Write,
    path::{Path, PathBuf},
};

pub mod differences;
pub mod directory;
pub mod initialize;


#[derive(Copy, Debug, Clone)]
pub enum SnapShotKind {
    Initialization = 0,
    Fix,
    Test,
}
