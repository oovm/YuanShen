use crate::{AuthorID, DirectoryEntry, ObjectID, SnapShotDirectory};
use blake3::Hash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Formatter},
    path::PathBuf,
    time::SystemTime,
};
use std::cmp::Ordering;

pub mod differences;
pub mod directory;


/// 快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapShot {
    /// 当前目录结构的 id
    pub directory: ObjectID,
    /// 快照的前驱节点, 可能没有, 或者一个, 或者多个
    pub previous: BTreeSet<ObjectID>,
    pub data: SnapShotData,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct SnapShotData {
    /// 快照类型, fix, test 或者其他
    pub kind: u32,
    /// The message added with the commit.
    pub message: String,
    /// The author ids of the commit.
    pub authors: BTreeSet<AuthorID>,
    // pub datetime: SystemTime
}

#[derive(Copy, Debug, Clone)]
pub enum SnapShotKind {
    Initialization = 0,
    Fix,
    Test,
}

impl Eq for SnapShot {}

impl PartialEq for SnapShot {
    fn eq(&self, other: &Self) -> bool {
        // 数据不加入校验
        self.directory.eq(&other.directory) && self.previous.eq(&other.previous)
    }
}
