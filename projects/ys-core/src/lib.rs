#![feature(fs_try_exists)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod objects;
mod snapshot;

const DOT_YUAN_SHEN: &'static str = ".ys";
const BRANCHES_DIRECTORY: &'static str = "branches";
const CURRENT_BRANCH_FILE: &'static str = "branch";
const CONFIG_BRANCH_FILE: &'static str = ".config";
const IGNORE_FILE: &'static str = ".ys.ignore";

pub use crate::{
    errors::{Result, YsError, YsErrorKind},
    objects::{
        author_id::AuthorID,
        ignore_rules::IgnoreRules,
        object_id::ObjectID,
        object_store::{file_system::LocalObjectStore, in_memory::MemoryObjectStore, ObjectStore},
    },
    snapshot::{
        differences,
        directory::{DirectoryEntry, SnapShotDirectory},
        initialize, SnapShot, SnapShotData,
    },
};
