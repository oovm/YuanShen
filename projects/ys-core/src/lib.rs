#![feature(fs_try_exists)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod objects;
mod snapshot;

pub use crate::{
    errors::{Result, YsError, YsErrorKind},
    objects::{
        object_id::ObjectID,
        author_id::AuthorID,
        object_store::{
            dot_directory::{DotYuanShen, InsertJson},
            file_system::LocalObjectStore,
            in_memory::MemoryObjectStore,
            ObjectStore,
        },
    },
    snapshot::{
        differences,
        directory::{DirectoryEntry, Ignores, SnapShotDirectory},
        SnapShot, SnapShotData,
    },
};
