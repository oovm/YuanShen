#![feature(fs_try_exists)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod objects;
mod snapshot;
mod directory;

pub use crate::errors::{Result};
pub use crate::errors::{YsErrorKind, YsError};
pub use crate::objects::object_id::ObjectID;
pub use crate::objects::object_store::{ObjectStore, in_memory::MemoryObjectStore, file_system::LocalObjectStore};
pub use crate::snapshot::SnapShot;