#![feature(fs_try_exists)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
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
    objects::{

    },
    snapshot::{
        differences,
        directory::{DirectoryEntry, SnapShotTree},
        initialize, 
    },
    traits::YuanShenClient
};


/// Create a test environment which returns the [Result<()>]
pub fn async_test<F>(future: F)
where
    F: std::future::Future<Output = std::result::Result<(), YsError>>,
{
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async { future.await.unwrap() })
}
