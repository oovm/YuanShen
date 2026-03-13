use crate::{
    DOT_YUAN_SHEN, DirectoryEntry,
    errors::YsError,
    objects::IgnoreRules,
    snapshot::directory::SnapShotTree,
    utils::{read_json, write_json},
};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub mod differences;
pub mod directory;
pub mod initialize;

#[allow(dead_code)]
#[derive(Copy, Debug, Clone)]
pub enum SnapShotKind {
    Initialization = 0,
    Fix,
    Test,
}
