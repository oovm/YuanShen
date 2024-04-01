use crate::{
    objects::{ObjectID, TextFile, TextIncrementalData},
    traits::BranchProxy,
    utils::{from_json, read_to_string, truncate_write},
    ObjectProxy, YsError, YsErrorKind, YuanShenObject,
};
use dashmap::DashMap;
pub use file_system::LocalDotYuanShen;
pub use in_memory::MemoryObjectPool;
use std::{
    fs::{create_dir, try_exists},
    path::PathBuf,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

mod file_system;
mod in_memory;
