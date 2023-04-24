use crate::{errors::YsError, snapshot::{directory::SnapShotDirectory}, AuthorID, DirectoryEntry, IgnoreRules, LocalObjectStore, ObjectID, ObjectStore, CURRENT_BRANCH_FILE, DOT_YUAN_SHEN, TreeID};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    fs::{create_dir, create_dir_all, read_dir, read_to_string, try_exists, File},
    future::Future,
    io::Write,
    path::{Path, PathBuf},
};

pub mod differences;
pub mod directory;
pub mod initialize;

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
