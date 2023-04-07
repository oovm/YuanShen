use std::collections::BTreeSet;
use std::fmt::Display;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::ObjectID;
use std::collections::BTreeMap;
use crate::{DirectoryEntry, SnapShotDirectory};

pub mod directory;
pub mod difference;

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
}

impl Eq for SnapShot {}


impl PartialEq for SnapShot {
    fn eq(&self, other: &Self) -> bool {
        // 数据不加入校验
        self.directory.eq(&other.directory) && self.previous.eq(&other.previous)
    }
}


