use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeSet, time::SystemTime};
use crate::objects::{AuthorID, ObjectID};
use crate::traits::YuanShenObject;

/// 快照
#[derive(Clone, Debug)]
pub struct Commit {
    pub datetime: SystemTime,
    /// 快照的前驱节点, 可能没有, 或者一个, 或者多个
    pub parents: Vec<CommitParent>,
    /// The author ids of the commit.
    pub authors: BTreeSet<AuthorID>,
}

#[derive(Copy, Clone, Debug)]
pub struct CommitParent {
    pub tree: ObjectID,
    pub modifier: ObjectID,
}

impl Serialize for CommitParent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("tree", &self.tree)?;
        map.serialize_entry("modifier", &self.modifier)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for CommitParent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl YuanShenObject for CommitParent {
    fn object_id(&self) -> ObjectID {
        todo!()
    }
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

impl Eq for Commit {}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        // 数据不加入校验
        todo!()
        // self.tree.eq(&other.tree) && self.parents.eq(&other.parents)
    }
}
