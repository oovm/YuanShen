#![feature(fs_try_exists)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod objects;
mod snapshot;
pub mod storage;

pub(crate) mod utils;

const DOT_YUAN_SHEN: &'static str = ".ys";

pub use crate::{
    errors::{Result, YsError, YsErrorKind},
    objects::{
        author_id::AuthorID, ignore_rules::IgnoreRules, object_id::ObjectID, object_store::file_system::LocalObjectStore,
    },
    snapshot::{
        differences,
        directory::{DirectoryEntry, SnapShotTree},
        initialize, Commit,
    },
};

/// Create a test environment which returns the [Result<()>]
pub fn async_test<F>(future: F)
where
    F: std::future::Future<Output = std::result::Result<(), YsError>>,
{
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async { future.await.unwrap() })
}
