use crate::{
    errors::YsError, objects::object_store::YuanShenObject, snapshot::directory::SnapShotTree, AuthorID, DirectoryEntry,
    IgnoreRules, LocalObjectStore, ObjectID, ObjectStore, DOT_YUAN_SHEN,
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
    time::SystemTime,
};
use crate::objects::commit_parent::CommitParent;

pub mod differences;
pub mod directory;
pub mod initialize;

/// 快照
#[derive(Clone, Debug)]
pub struct Commit {
    pub datetime: SystemTime,
    /// 快照的前驱节点, 可能没有, 或者一个, 或者多个
    pub parents: Vec<CommitParent>,
    /// The author ids of the commit.
    pub authors: BTreeSet<AuthorID>,
}



impl Serialize for Commit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Commit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl YuanShenObject for Commit {
    fn object_id(&self) -> ObjectID {
        todo!();
        // let mut hasher = blake3::Hasher::default();
        // hasher.update(self.tree.hash256.as_bytes());
        // for id in &self.parents {
        //     hasher.update(id.hash256.as_bytes());
        // }
        // for author in &self.extra.authors {
        //     hasher.update(author.hash256.as_bytes());
        // }
        // hasher.finalize().into()
    }
}

#[derive(Copy, Debug, Clone)]
pub enum SnapShotKind {
    Initialization = 0,
    Fix,
    Test,
}

impl Eq for Commit {}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        // 数据不加入校验
        todo!()
        // self.tree.eq(&other.tree) && self.parents.eq(&other.parents)
    }
}
