mod in_memory;

use crate::{ YsError, YsErrorKind};
pub use in_memory::MemoryObjectPool;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, future::Future};
use tokio::{fs::File, io::AsyncWriteExt};
use crate::objects::{ObjectID, TextFile};
