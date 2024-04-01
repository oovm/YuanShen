use crate::{
    errors::YsError,
    objects::{IgnoreRules, ObjectID},
    snapshot::directory::SnapShotTree,
    utils::{read_json, truncate_write, write_json},
    DirectoryEntry, DOT_YUAN_SHEN,
};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    fs::{create_dir, create_dir_all, read_dir, read_to_string, try_exists, File},
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
