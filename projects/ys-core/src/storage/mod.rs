mod file_system;
mod in_memory;

use crate::{
    objects::{ObjectID, StandaloneTextFile},
    YsError, YsErrorKind, YuanShenClient,
};
pub use file_system::LocalDotYuanShen;
pub use in_memory::MemoryObjectPool;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    fs::{create_dir, try_exists},
    future::Future,
};
use tokio::{fs::File, io::AsyncWriteExt};
