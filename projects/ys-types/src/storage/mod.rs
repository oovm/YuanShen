use crate::{
    objects::{ObjectID, TextFile, TextIncrementalData},
    traits::BranchProxy,
    utils::{copy, from_json, read_to_string, truncate_write},
    ObjectProxy, YsError, YsErrorKind, YuanShenObject,
};
use dashmap::DashMap;
pub use file_system::LocalDotYuanShen;
pub use in_memory::MemoryObjectPool;
use std::{
    fs::{create_dir, try_exists},
    path::{Path, PathBuf},
};
use tokio::{fs::File, io::AsyncWriteExt};

mod file_system;
mod in_memory;
