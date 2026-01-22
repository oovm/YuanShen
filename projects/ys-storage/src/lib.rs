#[cfg(feature = "fs")]
pub mod file_system;
#[cfg(feature = "mem")]
pub mod in_memory;

#[cfg(feature = "fs")]
pub use file_system::LocalDotYuanShen;
#[cfg(feature = "mem")]
pub use in_memory::MemoryObjectPool;

use ys_types::traits::{ObjectProxy, BranchProxy};

/// A unified trait that combines ObjectProxy and BranchProxy
pub trait StorageBackend: ObjectProxy + BranchProxy + Send + Sync {}
impl<T: ObjectProxy + BranchProxy + Send + Sync> StorageBackend for T {}
