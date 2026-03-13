#![doc = include_str!("readme.md")]

pub mod file_system;
pub mod in_memory;
#[cfg(feature = "limbo")]
pub mod limbo_fs;

pub use file_system::LocalDotYuanShen;
pub use in_memory::MemoryObjectPool;
#[cfg(feature = "limbo")]
pub use limbo_fs::LimboFsStorage;

use ys_types::{BranchProxy, GarbageCollect, ObjectProxy};

/// A unified trait that combines ObjectProxy and BranchProxy
pub trait StorageBackend: ObjectProxy + BranchProxy + Send + Sync {}

impl<T: ObjectProxy + BranchProxy + Send + Sync> StorageBackend for T {}

/// A unified trait that combines ObjectProxy, BranchProxy, and GarbageCollect
pub trait StorageBackendWithGc: ObjectProxy + BranchProxy + GarbageCollect + Send + Sync {}

impl<T: ObjectProxy + BranchProxy + GarbageCollect + Send + Sync> StorageBackendWithGc for T {}
