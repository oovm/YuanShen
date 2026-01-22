#![feature(fs_try_exists)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
pub mod objects;
mod snapshot;
pub mod storage;
mod traits;

pub(crate) mod utils;

const DOT_YUAN_SHEN: &'static str = ".ys";

pub use crate::{
    errors::{Result, YsError, YsErrorKind},
    snapshot::{
        differences,
        directory::{DirectoryEntry, SnapShotTree},
        initialize,
    },
    traits::{BranchProxy, ObjectProxy, YuanShenID, YuanShenObject},
    utils::async_test,
};
